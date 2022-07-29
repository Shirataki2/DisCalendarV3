import { getApiUrl } from '..'

export default defineEventHandler(async event => {
  const url = getApiUrl('/oauth2/logout')
  await sendRedirect(event, url)
})
