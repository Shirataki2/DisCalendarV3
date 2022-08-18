import { useTheme } from 'vuetify'

export const useUI = () => {
  const theme = useTheme()

  const toggleTheme = () => {
    theme.global.name.value =
      theme.global.name.value === 'myLightTheme'
        ? 'myDarkTheme'
        : 'myLightTheme'
    localStorage.setItem(
      'theme:dark',
      (theme.global.name.value === 'myDarkTheme').toString()
    )
  }
  return {
    theme,
    toggleTheme,
  }
}
