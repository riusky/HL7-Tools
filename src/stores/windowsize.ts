import { defineStore } from 'pinia'
import { ref } from 'vue'

export const useCounterStore = defineStore('counter', () => {
  const count = ref(0)
  let windowsHeight = ref()
  //  主题
  let isDark = ref(true)
  //  登录状态

  // 登录和注册页面的监控

  function increment() {
    count.value++
  }
  function changeWindowsHeight(height: number) {
    windowsHeight.value = height
  }
  return { count, increment, windowsHeight, changeWindowsHeight, isDark }
})


// ref() 就是 state 属性
// computed() 就是 getters
// function() 就是 actions 方法 (methods)



/* 
export const useCounterStore = defineStore('counter', {
  state: () => ({ count: 0 }),
  getters: {
    double: (state) => state.count * 2,
  },
  actions: {
    increment() {
      this.count++
    },
  },
})
你可以认为 state 是 store 的数据 (data)，
getters 是 store 的计算属性 (computed)，
而 actions 则是方法 (methods)。
*/