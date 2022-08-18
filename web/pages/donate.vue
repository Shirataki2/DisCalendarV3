<script lang="ts" setup>
import type Stripe from 'stripe'
import { getApiUrl } from '@/server'

useHead({
  title: '寄付',
})

definePageMeta({
  middleware: ['auth'],
})

const { getDonateProducts } = useStripe()
const { products, prices } = await getDonateProducts()

type Product = {
  id: string
  product_id: string | Stripe.Product | Stripe.DeletedProduct
  type: Stripe.Price.Type
  price: string
  label: string
  desc: string
}

const items = computed<Product[]>(() => {
  const donateProduct = products.data.find(p => p.id === 'prod_MFGRsUv9UpMvY4')
  if (!donateProduct) {
    return []
  }
  const donatePrices = prices.data
    .filter(p => p.product === donateProduct.id)
    .sort((a, b) => a.metadata.order.localeCompare(b.metadata.order))
  return donatePrices.map(p => ({
    id: p.id,
    product_id: p.product,
    type: p.type,
    price: p.unit_amount ? `${p.unit_amount}円` : `${p.metadata.price}`,
    label: p.metadata.label,
    desc: p.metadata.desc,
  }))
})

const startCheckoutSession = async (prod: Product) => {
  const url = await $fetch<string>(
    getApiUrl(`/payment/checkout?price_id=${prod.id}`)
  )
  window.location.href = url
}
</script>

<template>
  <div>
    <v-container>
      <v-row>
        <v-col cols="12">
          <v-card>
            <v-card-title class="text-center my-7">
              <strong class="text-h2"> 支援 </strong>
            </v-card-title>
            <v-card-text>
              <p class="my-4">
                Lorem ipsum dolor sit amet consectetur adipisicing elit. Animi
                atque sed cumque, dolorem eos adipisci, quo rerum nam ducimus
                dignissimos consequuntur magni fugit incidunt ullam cupiditate
                nostrum aliquid officiis totam!
              </p>
              <v-container>
                <v-row>
                  <v-col v-for="item in items" :key="item.id" cols="12" sm="6">
                    <v-card
                      variant="outlined"
                      @click="startCheckoutSession(item)"
                    >
                      <v-card-title class="text-center">
                        <span class="headline">{{ item.label }}</span>
                      </v-card-title>
                      <p class="text-center price">
                        <span class="subheading">{{ item.price }}</span>
                      </p>
                      <v-card-text>
                        {{ item.desc }}
                      </v-card-text>
                    </v-card>
                  </v-col>
                </v-row>
              </v-container>
            </v-card-text>
          </v-card>
        </v-col>
      </v-row>
    </v-container>
  </div>
</template>

<style scoped lang="scss">
.price {
  font-size: 2rem;
  margin: 0.7rem 0;
  color: cornflowerblue;
}
</style>
