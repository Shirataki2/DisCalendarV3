export const getApiUrl = (url: string): string => {
  // const baseUrl = process.env.API_ENDPOINT || 'http://localhost:15000'
  const baseUrl = process.client ? '/v3' : 'http://api:5000'
  return `${baseUrl}${url}`
}

/**
 *  Get the docker internal url for the given path.
 */
export const getInnerApiUrl = (url: string): string => {
  const baseUrl = 'http://api:5000'
  return `${baseUrl}${url}`
}
