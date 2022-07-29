export type AuthData = {
  access_token: string
  refresh_token: string
  expires_in: number
  token_type: 'Bearer'
  scope: string
}

export type AuthorizePayload = {
  client_id: string
  client_secret: string
  redirect_uri: string
  grant_type: string
  code: string
}
