<svelte:options runes={true} />


<script lang="ts">

</script>


<div class="grid grid-rows-2 h-full w-full">
    <div>SETTINGS</div>
    <div class="grid grid-cols-2">
        <div class="flex flex-col w-full h-full">
            <h3 class="mx-auto text-2xl">Payments</h3>
                <a href="/api/v1/payments/stripe/onboard">Stripe Setup</a>
        </div>
    </div>
</div>
