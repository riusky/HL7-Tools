<script setup lang="ts">
// 禁止右键和检查
//禁止F12
// document.onkeydown = function (event: any) {
//     var winEvent: any = window.event
//     if (winEvent && winEvent.keyCode == 123) {
//         event.keyCode = 0
//         event.returnValue = false
//     }
//     if (winEvent && winEvent.keyCode == 13) {
//         winEvent.keyCode = 505
//     }
// }

// //屏蔽右键菜单
// document.oncontextmenu = function (event: any) {
//     if (window.event) {
//         event = window.event
//     }
//     try {
//         var the = event.srcElement
//         if (
//             !(
//                 (the.tagName == 'INPUT' && the.type.toLowerCase() == 'text') ||
//                 the.tagName == 'TEXTAREA'
//             )
//         ) {
//             return false
//         }
//         return true
//     } catch (e) {
//         return false
//     }
// }


// This starter template is using Vue 3 <script setup> SFCs
// Check out https://vuejs.org/api/sfc-script-setup.html#script-setup
// import Greet from "@components/Greet.vue";
import { appWindow } from '@tauri-apps/api/window'
import { toggleDark } from "@/composables";
// import { storeToRefs } from 'pinia'
import { useCounterStore } from '@/stores/windowsize'
import { ElMessage } from 'element-plus'
import { useDark } from "@vueuse/core";
import Header from '@/components/layout/Header.vue';

import { watch, ref, onMounted } from 'vue'
import { gobackUrl } from '@/main'
// 可以在组件中的任意位置访问 `store` 变量 ✨
const store = useCounterStore()

const sidebar = ref(false)
const isDark = useDark();
let showBack = ref(false)
const router = useRouter()
let onTop = ref(false)


function toggleDarkDark() {
  toggleDark();
  store.isDark = !store.isDark
  if (store.isDark) {
    document.body.classList.add('dark');
  } else {
    document.body.classList.remove('dark');
  }
}

function changeSidebar() {
  if (window.innerWidth < 768) {
    ElMessage({
      message: '窗口太小已禁用侧边栏展开.请调整窗口大小后重试!',
      type: 'warning',
    })
    sidebar.value = false
    return
  }
  const sideBar = document.querySelector('.sidebar');
  sidebar.value = !sidebar.value
  sideBar?.classList.toggle('close');
}

function getCharCount(str: string, char: string | RegExp) {
  console.log('str', str)
  var regex = new RegExp(char, 'g') // 使用g表示整个字符串都要匹配
  var result = str.match(regex) //match方法可在字符串内检索指定的值，或找到一个或多个正则表达式的匹配。
  var count = !result ? 0 : result.length
  console.log('count', count)
  return count
}

watch(() => router.currentRoute.value,
  (newValue: any) => {
    console.log('newValue', newValue)
    if (newValue && getCharCount(newValue.fullPath, '/') > 1) {
      showBack.value = true;
    } else {
      showBack.value = false;
    }
  },
  { immediate: true }
)

function setAlwaysOnTop() {
  onTop.value = !onTop.value
  appWindow.setAlwaysOnTop(onTop.value)
}

onMounted(() => {
  store.isDark = isDark.value
  if (isDark.value) {
    document.body.classList.add('dark');
  } else {
    document.body.classList.remove('dark');
  }
  document.getElementById('titlebar-minimize')!.addEventListener('click', () => appWindow.minimize())
  document.getElementById('titlebar-maximize')!.addEventListener('click', () => appWindow.toggleMaximize())
  document.getElementById('titlebar-close')!.addEventListener('click', () => appWindow.close())

  const sideLinks = document.querySelectorAll('.sidebar .side-menu li a:not(.logout)');

  sideLinks.forEach(item => {
    const li = item.parentElement;
    item.addEventListener('click', () => {
      sideLinks.forEach(i => {
        i?.parentElement?.classList.remove('active');
      })
      li?.classList.add('active');
    })
  });

  const sideBar = document.querySelector('.sidebar');

  const searchBtn = document.querySelector('.content nav form .form-input button');
  const searchBtnIcon = document.querySelector('.content nav form .form-input button .bx');
  const searchForm = document.querySelector('.content nav form');

  searchBtn?.addEventListener('click', function (e) {
    if (window.innerWidth < 576) {
      e.preventDefault;
      searchForm?.classList.toggle('show');
      if (searchForm?.classList.contains('show')) {
        searchBtnIcon?.classList.replace('bx-search', 'bx-x');
      } else {
        searchBtnIcon?.classList.replace('bx-x', 'bx-search');
      }
    }
  });

  window.addEventListener('resize', () => {
    if (window.innerWidth < 768) {
      sideBar?.classList.add('close');
      sidebar.value = true
    } else {
      sideBar?.classList.remove('close');
      sidebar.value = false
    }
    if (window.innerWidth > 576) {
      searchBtnIcon?.classList.replace('bx-x', 'bx-search');
      searchForm?.classList.remove('show');
    }
  });

  const toggler = <HTMLInputElement>document.getElementById('theme-toggle');

  toggler?.addEventListener('change', function () {
    if (this.checked) {
      document.body.classList.add('dark');
      toggleDarkDark()
    } else {
      document.body.classList.remove('dark');
      toggleDarkDark()

    }
  });


})

</script>

<template>
  <!-- Sidebar -->
  <Header />
  <div class="content">
    <!-- Navbar -->
    <nav data-tauri-drag-region>
      <el-icon v-show="!sidebar" @click="changeSidebar" class='bx' :size="25">
        <Fold />
      </el-icon>
      <el-icon v-show="sidebar" @click="changeSidebar" class='bx' :size="25">
        <Expand />
      </el-icon>
      <div class="titlebar">



        <div v-if="showBack" class="titlebar-button" @click="gobackUrl">
          <!-- <img src="https://api.iconify.design/mdi:window-minimize.svg" alt="minimize" /> -->
          <!-- <el-icon style="color: aliceblue;"><Minus /></el-icon> -->
          <el-icon  :size="20" style="color: (store.isDark ? '#030b25' : '#DCDFE6');">
            <Back />
          </el-icon>
        </div>

        <div class="titlebar-button-top" @click="setAlwaysOnTop" :style="{'background': onTop?'#06b139':''}">
          <el-icon :size="18" style="color: (store.isDark ? '#030b25' : '#DCDFE6');"><Stamp /></el-icon>
        </div>
        
        <div class="titlebar-button" @click="toggleDarkDark">
          <el-icon  v-show="!store.isDark" :size="20"
            style="color: (store.isDark ? '#030b25' : '#DCDFE6');">
            <Moon />
          </el-icon>
          <el-icon v-show="store.isDark" :size="20"
            style="color: (store.isDark ? '#030b25' : '#DCDFE6');">
            <Sunny />
          </el-icon>
        </div>
        <div class="titlebar-button" id="titlebar-minimize">
          <el-icon>
            <SemiSelect />
          </el-icon>
        </div>
        <div class="titlebar-button" id="titlebar-maximize">
          <el-icon>
            <span class="iconfont icon-fullscreen"></span>
          </el-icon>
        </div>
        <div class="titlebar-button" id="titlebar-close">
          <el-icon>
            <CloseBold />
          </el-icon>
        </div>
      </div>
    </nav>
    <!-- End of Navbar -->
    <main>
      <router-view></router-view>
    </main>

  </div>
</template>

<style scoped>
.titlebar {
  justify-content: center;
  align-items: center;
  user-select: none;
  display: flex;
  justify-content: flex-end;
  position: absolute;
  top: 25;
  /* left: 0; */
  right: 0;
}

.icon {
  width: 1em;
  height: 1em;
  vertical-align: -0.15em;
  fill: currentColor;
  overflow: hidden;
  /* color: aliceblue; */
  color: var(--dark);
}

.logo.vite:hover {
  filter: drop-shadow(0 0 2em #747bff);
}

.logo.vue:hover {
  filter: drop-shadow(0 0 2em #249b73);
}

.titlebar-button {
  display: inline-flex;
  justify-content: center;
  align-items: center;
  width: 30px;
  height: 30px;
}

.titlebar-button-top {
  display: inline-flex;
  justify-content: center;
  align-items: center;
  width: 30px;
  height: 30px;
}

.titlebar-button:hover {
  background: #06b139;
}
</style>
