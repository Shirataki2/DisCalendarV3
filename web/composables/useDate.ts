import dayjs from 'dayjs'

export enum DateView {
  Years = 'years',
  Year = 'year',
  Month = 'month',
  Week = 'week',
  Day = 'day',
}

export const useDate = () => {
  const date = useState('date', () => new Date())
  const view = useState('view', () => DateView.Month)

  const setDate = (newDate: Date) => {
    date.value = newDate
  }

  const formattedDate = computed(() => {
    switch (view.value) {
      case DateView.Years:
        return dayjs(date.value).format('YYYY')
      case DateView.Year:
        return dayjs(date.value).format('YYYY')
      case DateView.Month:
        return dayjs(date.value).format('YYYY年 M月')
      case DateView.Week:
        return formatWeekView(date.value)
      case DateView.Day:
        return dayjs(date.value).format('YYYY年 M月 D日')
    }
  })
  return {
    date,
    view,
    setDate,
    formattedDate,
  }
}

const formatWeekView = (date: Date) => {
  // calculate start and end of week
  const start = dayjs(date).startOf('week')
  const end = dayjs(date).endOf('week')
  // if start and end are same month, return start
  if (start.month() === end.month()) {
    return start.format('YYYY年 M月')
  } else {
    return `${start.format('YYYY年 M月')} - ${end.format('M月')}`
  }
}
