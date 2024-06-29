use crate::helpers::auth::UserState;
use crate::helpers::AppResult;
use axum::body::Body;
use axum::extract::{FromRef, FromRequest, Path, Request, State};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Redirect, Response};
use axum::{async_trait, debug_handler, Extension};
use serde_derive::Deserialize;
use serde_derive::Serialize;
use serde_json::{json, Value};
use shared::db::account::FullAccount;
use shared::db::model::FullModel;
use shared::db::transactions::{CreateTransaction, FullTransaction};
use shared::AppState;
use std::sync::Arc;
use stripe::Object;
use tracing::{debug, error};
use uuid::Uuid;

#[debug_handler]
pub async fn onboard(
    Extension(claims): Extension<UserState>,
    State(state): State<Arc<AppState>>,
) -> AppResult<impl IntoResponse> {
    let stripe_client = match &state.stripe {
        Some(d) => d,
        None => return Ok((StatusCode::NOT_IMPLEMENTED, "Payments not available").into_response()),
    };
    let account = stripe::Account::create(
        stripe_client,
        stripe::CreateAccount {
            ..Default::default()
        },
    )
    .await?;
    FullAccount::link_stripe_id(&claims.sub, &account.id, state.pool.clone()).await?;
    // TODO save account ID
    // Ok((
    //     StatusCode::OK,
    //     json!({"account_id": account.id}).to_string(),
    // )
    //     .into_response())
    let account_link = stripe::AccountLink::create(
        stripe_client,
        stripe::CreateAccountLink {
            account: account.id,
            type_: stripe::AccountLinkType::AccountOnboarding,
            refresh_url: Some(&format!(
                "/api/v1/payments/onboard/{}",
                claims.sub.to_string()
            )),
            return_url: Some(&format!(
                "/account/payments/onboarding/complete/{}",
                claims.sub.to_string()
            )),
            collect: None,
            collection_options: None,
            expand: &vec![],
        },
    )
    .await?;
    Ok(Redirect::temporary(&account_link.url).into_response())
    // Ok(StatusCode::OK.into_response())
}

#[debug_handler]
pub async fn pay(
    Extension(claims): Extension<UserState>,
    State(state): State<Arc<AppState>>,
    Path(model_id): Path<Uuid>,
) -> AppResult<impl IntoResponse> {
    let stripe_client = match &state.stripe {
        Some(d) => d,
        None => return Ok((StatusCode::NOT_IMPLEMENTED, "Payments not available").into_response()),
    };
    let model = FullModel::get_by_id_and_public_and_paid(&model_id, state.pool.clone()).await?;
    let cost: i64 = 10_00;
    let application_percent: i64 = 10;
    let fee: i64 = cost * application_percent;
    let session = stripe::CheckoutSession::create(
        stripe_client,
        stripe::CreateCheckoutSession {
            line_items: Some(vec![stripe::CreateCheckoutSessionLineItems {
                price_data: Some(stripe::CreateCheckoutSessionLineItemsPriceData {
                    currency: stripe::Currency::EUR,
                    unit_amount: Some(cost),
                    product_data: Some(
                        stripe::CreateCheckoutSessionLineItemsPriceDataProductData {
                            name: "Product Name".to_string(),
                            ..Default::default()
                        },
                    ),
                    recurring: None,
                    ..Default::default()
                }),
                quantity: Some(1),
                adjustable_quantity: Some(
                    stripe::CreateCheckoutSessionLineItemsAdjustableQuantity {
                        enabled: false,
                        ..Default::default()
                    },
                ),
                dynamic_tax_rates: None,
                price: None,
                tax_rates: None,
            }]),
            payment_intent_data: Some(stripe::CreateCheckoutSessionPaymentIntentData {
                application_fee_amount: Some(fee),
                ..Default::default()
            }),
            mode: Some(stripe::CheckoutSessionMode::Payment),
            success_url: Some("/payments/complete/{CHECKOUT_SESSION_ID}"),
            ..Default::default()
        },
    )
    .await?;

    Ok(Redirect::temporary(&session.url.unwrap()).into_response())
}

// Adapted from https://github.com/arlyon/async-stripe/blob/master/examples/webhook-axum.rs
struct StripeEvent(stripe::Event);

#[async_trait]
impl<S> FromRequest<S> for StripeEvent
where
    Arc<AppState>: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = Response;

    async fn from_request(req: Request<Body>, state: &S) -> Result<Self, Self::Rejection> {
        let app_state = Arc::<AppState>::from_ref(state);
        let stripe_sig = match &app_state.env.stripe {
            Some(d) => &d.webhook_key,
            None => return Err(StatusCode::NOT_IMPLEMENTED.into_response()),
        };
        let signature = if let Some(sig) = req.headers().get("stripe-signature") {
            sig.to_owned()
        } else {
            return Err(StatusCode::BAD_REQUEST.into_response());
        };

        let payload = String::from_request(req, state)
            .await
            .map_err(IntoResponse::into_response)?;

        Ok(Self(
            stripe::Webhook::construct_event(&payload, signature.to_str().unwrap(), &stripe_sig)
                .map_err(|_| StatusCode::BAD_REQUEST.into_response())?,
        ))
    }
}

#[axum::debug_handler]
async fn handle_webhook(
    State(state): State<Arc<AppState>>,
    StripeEvent(event): StripeEvent,
) -> AppResult<impl IntoResponse> {
    match event.type_ {
        stripe::EventType::CheckoutSessionCompleted => {
            if let stripe::EventObject::CheckoutSession(session) = event.data.object {
                debug!(
                    "Received CheckoutSessionCompleted webhook with id: {:?}",
                    session.id
                );
                FullTransaction::mark_completed_true(session.id().as_str(), state.pool.clone())
                    .await?;
            }
        }
        // stripe::EventType::AccountUpdated => {
        //     if let stripe::EventObject::Account(account) = event.data.object {
        //         debug!(
        //             "Received account updated webhook for account: {:?}",
        //             account.id
        //         );
        //     }
        // }
        stripe::EventType::CheckoutSessionAsyncPaymentSucceeded => {
            if let stripe::EventObject::CheckoutSession(session) = event.data.object {
                debug!(
                    "Received CheckoutSessionAsyncPaymentSucceeded webhook with id: {:?}",
                    session.id
                );
                FullTransaction::mark_payment_success_true(
                    session.id().as_str(),
                    state.pool.clone(),
                )
                .await?;
            }
        }
        stripe::EventType::CheckoutSessionAsyncPaymentFailed => {
            if let stripe::EventObject::CheckoutSession(session) = event.data.object {
                debug!(
                    "Received CheckoutSessionAsyncPaymentFailed webhook with id: {:?}",
                    session.id
                );
                FullTransaction::mark_payment_success_false(
                    session.id().as_str(),
                    state.pool.clone(),
                )
                .await?;
            }
        }
        _ => error!("Unknown event encountered in webhook: {:?}", event.type_),
    }
    Ok(StatusCode::OK.into_response())
}
