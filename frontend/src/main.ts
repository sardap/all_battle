import './assets/main.css'

import { createApp } from 'vue'
import App from './App.vue'
import router from './router'
import { createI18n } from 'vue-i18n'
import { messages } from './messages'

function getLocale(): string {
  for (const locale of navigator.languages) {
    if (locale == 'kr' || locale == 'ko' || locale == 'ko-KR') {
      return 'kr'
    }
  }

  return 'en'
}

export const i18n = createI18n({
  locale: getLocale(),
  fallbackLocale: 'en',
  messages: messages
})

const app = createApp(App)

app.use(router)
app.use(i18n)

app.mount('#app')
