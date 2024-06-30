## Stripe
If you want to set up [Stripe](https://stripe.com), you'll have to create an account there.

Now, you'll want to get your `API-KEY` and your `Account-ID`.


The API-Key can be obtained [here for in the test mode](https://dashboard.stripe.com/test/apikeys) and [here in the live mode](https://dashboard.stripe.com/apikeys).


You can get your user-id [here when you scroll down](https://dashboard.stripe.com/settings/user).


You'll also have to configure your webhooks. For that, use the stripe cli in the dev mode and in the production mode,
add a new webhook [here](https://dashboard.stripe.com/webhooks/create).


> [!IMPORTANT]
> Make sure to select in **`connected accounts`** under the description!!!

For both the production and the dev setup, the endpoint-url is

```
https://{HOST}/api/v1/payments/stripe/webhook
```

so, in the dev-mode it should be `localhost:8080/api/v1/payments/stripe/webhook`.

Now, set the following env-variables:

```env
STRIPE__KEY="sk_test_xxx"
STRIPE__WEBHOOK_KEY="whsec_xxx"
STRIPE__PLATFORM_FEE_PERCENT="0"
STRIPE__ACCOUNT_ID= "acct_xxx"
```

The fee percent tells how high the fee should be, that you as the admin gets, if any (leave it at 0 for a good community!).
