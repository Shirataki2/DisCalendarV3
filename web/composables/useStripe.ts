import type Stripe from 'stripe'
import { getApiUrl } from '@/server'

export type StripeProducts = {
  products: Stripe.ApiList<Stripe.Product>
  prices: Stripe.ApiList<Stripe.Price>
}

export const useStripe = () => {
  const getDonateProducts = async () => {
    const products = await $fetch<StripeProducts>(getApiUrl('/payment/donate'))
    return products
  }
  return {
    getDonateProducts,
  }
}
