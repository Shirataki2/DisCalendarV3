import { getApiUrl } from '..'

export default defineEventHandler(async event => {
  const url = getApiUrl('/oauth2/login')
  await sendRedirect(event, url)
})
