export const useDialog = () => {
  const dialog = ref(false)
  const setDialog = (value: boolean) => {
    dialog.value = value
  }
  const toggleDialog = () => {
    dialog.value = !dialog.value
  }
  return {
    dialog,
    setDialog,
    toggleDialog,
  }
}
