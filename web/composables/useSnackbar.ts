export const useSnackbar = () => {
  const showSnackbar = useState('showSnackbar', () => false)
  const message = useState('message', () => '')
  const color = useState('color', () => 'success')
  const timeout = useState('timeout', () => 3000)

  const toggleSnackbar = () => {
    showSnackbar.value = !showSnackbar.value
  }
  return {
    showSnackbar,
    message,
    color,
    timeout,
    toggleSnackbar,
  }
}
