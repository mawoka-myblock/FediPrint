use crate::helpers::auth::UserState;
use crate::helpers::AppResult;
use axum::body::Body;
use axum::extract::{FromRef, FromRequest, Path, Request, State};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Redirect, Response};
use axum::{async_trait, debug_handler, Extension};
use reqwest::Client;
use shared::db::account::FullAccount;
use shared::db::model::FullModel;
use shared::db::profile::FullProfile;
use shared::db::transactions::{CreateTransaction, FullTransaction};
use shared::AppState;
use std::str::FromStr;
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
    let db_account = FullAccount::get_by_id(&claims.sub, state.pool.clone()).await?;
    if db_account.stripe_id.is_some() {
        return Ok(Redirect::temporary("/api/v1/payments/stripe/dashboard").into_response());
    }
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
                "{}/api/v1/payments/onboard/{}",
                state.env.public_url, claims.sub
            )),
            return_url: Some(&format!(
                "{}/account/payments/onboarding/complete/{}",
                state.env.public_url, claims.sub
            )),
            collect: None,
            collection_options: None,
            expand: &[],
        },
    )
    .await?;
    Ok(Redirect::temporary(&account_link.url).into_response())
    // Ok(StatusCode::OK.into_response())
}

#[debug_handler]
pub async fn open_dashboard(
    Extension(_claims): Extension<UserState>,
    State(_state): State<Arc<AppState>>,
) -> AppResult<impl IntoResponse> {
    Ok(Redirect::temporary("https://dashboard.stripe.com").into_response())
}

#[debug_handler]
pub async fn pay(
    Extension(claims): Extension<UserState>,
    State(state): State<Arc<AppState>>,
    Path(model_id): Path<Uuid>,
) -> AppResult<impl IntoResponse> {
    let mut stripe_client = match &state.stripe {
        Some(d) => d.clone(),
        None => return Ok((StatusCode::NOT_IMPLEMENTED, "Payments not available").into_response()),
    };

    let model = FullModel::get_by_id_and_public_and_paid(&model_id, state.pool.clone()).await?;
    if model.cost.is_none() {
        return Ok((StatusCode::NOT_FOUND, "Model isn't available for purchase").into_response());
    }
    if model.currency.is_none() {
        return Ok((StatusCode::NOT_FOUND, "Model isn't available for purchase").into_response());
    }
    let seller_profile = FullProfile::get_by_id(&model.profile_id, state.pool.clone()).await?;
    let seller_account =
        FullAccount::get_by_profile_id(&seller_profile.id, state.pool.clone()).await?;
    stripe_client = stripe_client.with_stripe_account(
        stripe::AccountId::from_str(&seller_account.stripe_id.unwrap()).unwrap(),
    );
    let cost: i64 = model.cost.unwrap().into();
    let currency = match stripe::Currency::from_str(&model.currency.unwrap()) {
        Ok(d) => d,
        Err(_) => {
            error!("Model currency couldn't be loaded (Model-id: {}", &model.id);
            return Ok((
                StatusCode::NOT_FOUND,
                "Currency for model couldn't be loaded",
            )
                .into_response());
        }
    };
    let stripe_account_id = &state.env.stripe.as_ref().unwrap().account_id;
    let fee: i64 = cost * i64::from(state.env.stripe.as_ref().unwrap().platform_fee_percent); // Unwrap is fine, as the client at the beginning only exists when stripe is configured
    let session = stripe::CheckoutSession::create(
        &stripe_client,
        stripe::CreateCheckoutSession {
            line_items: Some(vec![stripe::CreateCheckoutSessionLineItems {
                price_data: Some(stripe::CreateCheckoutSessionLineItemsPriceData {
                    currency,
                    unit_amount: Some(cost),
                    product_data: Some(
                        stripe::CreateCheckoutSessionLineItemsPriceDataProductData {
                            name: model.title,
                            description: Some(model.summary),
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
            success_url: Some(&format!(
                "{}/payments/complete/{{CHECKOUT_SESSION_ID}}",
                state.env.public_url
            )),
            ..Default::default()
        },
    )
    .await?;

    CreateTransaction {
        buyer_account: claims.sub,
        buyer_profile: claims.profile_id,
        model_id: model.id,
        seller_profile: seller_profile.id,
        stripe_id: session.id().to_string(),
    }
    .create(state.pool.clone())
    .await?;

    Ok(Redirect::temporary(&session.url.unwrap()).into_response())
}

// Adapted from https://github.com/arlyon/async-stripe/blob/master/examples/webhook-axum.rs
pub struct StripeEvent(stripe::Event);

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
            stripe::Webhook::construct_event(&payload, signature.to_str().unwrap(), stripe_sig)
                .map_err(|_| StatusCode::BAD_REQUEST.into_response())?,
        ))
    }
}

#[axum::debug_handler]
pub async fn handle_webhook(
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
                if session.payment_status == stripe::CheckoutSessionPaymentStatus::Paid {
                    FullTransaction::mark_payment_success_true(
                        session.id().as_str(),
                        state.pool.clone(),
                    )
                    .await?;
                }
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
