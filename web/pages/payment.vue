<script lang="ts" setup>
import { getApiUrl } from '~~/server'

const simpleCardLayout = {
  cols: 12,
  sm: 10,
  md: 8,
  lg: 6,
  'offset-sm': 1,
  'offset-md': 2,
  'offset-lg': 3,
}

const customerId = ref('')

const route = useRoute()
const paymentSuccessed =
  typeof route.query.status === 'string' && route.query.status === 'success'

onMounted(async () => {
  if (
    paymentSuccessed &&
    route.query.session_id &&
    typeof route.query.session_id === 'string'
  ) {
    const sessionId = route.query.session_id
    const { customer_id: id } = await $fetch<{
      customer_id: string
    }>(getApiUrl(`/payment/register?session_id=${sessionId}`), {
      method: 'POST',
    })
    customerId.value = id
  }
})
</script>

<template>
  <v-container>
    <v-row>
      <v-col v-bind="simpleCardLayout">
        <v-card>
          <v-card-title class="my-7 text-center">
            <strong v-if="paymentSuccessed" class="text-h5">
              決済が完了しました
            </strong>
            <strong v-else class="text-h5"> 決済をキャンセルしました </strong>
          </v-card-title>
          <v-card-text>
            <p v-if="paymentSuccessed" class="mb-8">
              以下のお客様IDはお客様の決済情報を確認するために使用します。
              このページを離れてもマイページより、確認することが可能です。
            </p>
            <v-text-field
              v-if="paymentSuccessed"
              v-model="customerId"
              label="お客様ID"
              variant="outlined"
              readonly
            ></v-text-field>
            <v-btn to="/" block color="info"> ホームに戻る </v-btn>
          </v-card-text>
        </v-card>
      </v-col>
    </v-row>
  </v-container>
</template>

<style scoped></style>
