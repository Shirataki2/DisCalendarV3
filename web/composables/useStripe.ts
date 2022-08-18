import type Stripe from 'stripe'
import { getApiUrl } from '@/server'

export type StripeProducts = {
  products: Stripe.ApiList<Stripe.Product>
  prices: Stripe.ApiList<Stripe.Price>
}

export const useStripe = () => {
  console.info(5)
  const getDonateProducts = async () => {
    console.info(getApiUrl('/payment/donate'))
    const products = await $fetch<StripeProducts>(getApiUrl('/payment/donate'))
    console.info(7)
    return products
  }
  return {
    getDonateProducts,
  }
}
