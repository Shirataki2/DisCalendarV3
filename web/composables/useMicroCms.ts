import type { MicroCMSListResponse, MicroCMSDate } from 'microcms-js-sdk'

type Category =
  | {
      id: string
      name: string
    }
  | MicroCMSDate

type Notification =
  | {
      id: string
      title: string
      content: string
      category: Category
    }
  | MicroCMSDate

export const useMicroCms = () => {
  const ctx = useRuntimeConfig()

  const baseHeader = {
    baseURL: ctx.microcmsServiceDomain,
    headers: {
      'X-API-KEY': ctx.microcmsApiKey,
    },
  }

  async function getNotifications() {
    const result = await useFetch<MicroCMSListResponse<Notification>>(
      '/notifications',
      baseHeader
    )
    return result
  }
  return {
    getNotifications,
  }
}
