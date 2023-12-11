<script setup lang="ts">
// This starter template is using Vue 3 <script setup> SFCs
// Check out https://vuejs.org/api/sfc-script-setup.html#script-setup
// import Greet from "@components/Greet.vue";
import { appWindow } from '@tauri-apps/api/window'
import { toggleDark } from "@/composables";
// import { storeToRefs } from 'pinia'
import { useCounterStore } from '@/stores/windowsize'
// 可以在组件中的任意位置访问 `store` 变量 ✨
const store = useCounterStore()

function toggleDarkDark() {
    toggleDark();
    store.isDark = !store.isDark
    if (store.isDark) {
      document.body.classList.add('dark');
    } else {
      document.body.classList.remove('dark');
    }
}


onMounted(() => {
  document.body.classList.add('dark');
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

  const menuBar = document.querySelector('nav .bx.bx-menu');
  const sideBar = document.querySelector('.sidebar');

  menuBar?.addEventListener('click', () => {
    sideBar?.classList.toggle('close');
  });

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
    } else {
      sideBar?.classList.remove('close');
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
  <div class="sidebar">
    <a href="#" class="logo">
      <i class='bx bx-code-alt'></i>
      <div class="logo-name"><span>Asmr</span>Prog</div>
    </a>
    <ul class="side-menu">
      <li><a href="#"><el-icon class='bx bxs-dashboard'>
            <Menu />
          </el-icon>首页</a></li>
      <li><a href="#"><el-icon class='bx bx-store-alt'>
            <Document />
          </el-icon>任务清单</a></li>
      <li class="active"><a href="#"><el-icon class='bx bx-analyse'>
            <List />
          </el-icon>备忘</a></li>
      <li><a href="#"><el-icon class='bx bx-message-square-dots'>
            <Promotion />
          </el-icon>通讯</a></li>
      <li><a href="#"><el-icon class='bx bx-group'>
            <Comment />
          </el-icon>聊天</a></li>
      <li><a href="#"><el-icon class='bx bx-cog'>
            <Tools />
          </el-icon>设置</a></li>
    </ul>
  </div>

  <div class="content">
    <!-- Navbar -->
    <nav data-tauri-drag-region>
      <el-icon class='bx bx-menu' :size="25">
        <Fold />
      </el-icon>
      <div class="titlebar">

<!-- 
        <input type="checkbox" id="theme-toggle" hidden>
        <label for="theme-toggle" class="theme-toggle"></label> -->


        <div class="titlebar-button">
                <!-- <img src="https://api.iconify.design/mdi:window-minimize.svg" alt="minimize" /> -->
                <!-- <el-icon style="color: aliceblue;"><Minus /></el-icon> -->
                <el-icon @click="toggleDarkDark" v-show="!store.isDark" :size="20" style="color: (store.isDark ? '#030b25' : '#DCDFE6');">
                    <Moon />
                </el-icon>

                <el-icon @click="toggleDarkDark" v-show="store.isDark" :size="20" style="color: (store.isDark ? '#030b25' : '#DCDFE6');">
                    <Sunny />
                </el-icon>
            </div>

        <div class="titlebar-button" id="titlebar-minimize">
          <!-- <img src="https://api.iconify.design/mdi:window-minimize.svg" alt="minimize" /> -->
          <!-- <el-icon style="color: aliceblue;"><Minus /></el-icon> -->
          <el-icon style="color: (store.isDark ? '#030b25' : '#DCDFE6');">
            <SemiSelect />
          </el-icon>
        </div>
        <div class="titlebar-button" id="titlebar-maximize">
          <!-- <img src="https://api.iconify.design/mdi:window-maximize.svg" alt="maximize" /> -->
          <el-icon style="color: (store.isDark ? '#030b25' : '#DCDFE6');">
            <FullScreen />
          </el-icon>
        
        </div>
        <div class="titlebar-button" id="titlebar-close">
          <!-- <el-icon style="color: aliceblue;"><Close /></el-icon> -->
``
          <el-icon style="color: (store.isDark ? '#030b25' : '#DCDFE6');">
            <CloseBold />
          </el-icon>
          <!-- <img src="https://api.iconify.design/mdi:close.svg" alt="close" /> -->
        </div>

      </div>


    </nav>

    <!-- End of Navbar -->

    <main>
      <div class="header">
        <div class="left">
          <h1>Dashboard</h1>
          <ul class="breadcrumb">
            <li><a href="#">
                Analytics
              </a></li>
            /
            <li><a href="#" class="active">Shop</a></li>
          </ul>
        </div>
        <a href="#" class="report">
          <i class='bx bx-cloud-download'></i>
          <span>Download CSV</span>
        </a>
      </div>
      <el-row class="mb-4">
    <el-button>Default</el-button>
    <el-button type="primary">Primary</el-button>
    <el-button type="success">Success</el-button>
    <el-button type="info">Info</el-button>
    <el-button type="warning">Warning</el-button>
    <el-button type="danger">Danger</el-button>
  </el-row>
      <!-- Insights -->
      <ul class="insights">
        <li>
          <i class='bx bx-calendar-check'></i>
          <span class="info">
            <h3>
              1,074
            </h3>
            <p>Paid Order</p>
          </span>
        </li>
        <li><i class='bx bx-show-alt'></i>
          <span class="info">
            <h3>
              3,944
            </h3>
            <p>Site Visit</p>
          </span>
        </li>
        <li><i class='bx bx-line-chart'></i>
          <span class="info">
            <h3>
              14,721
            </h3>
            <p>Searches</p>
          </span>
        </li>
        <li><i class='bx bx-dollar-circle'></i>
          <span class="info">
            <h3>
              $6,742
            </h3>
            <p>Total Sales</p>
          </span>
        </li>
      </ul>
      <!-- End of Insights -->

      <div class="bottom-data">
        <div class="orders">
          <div class="header">
            <i class='bx bx-receipt'></i>
            <h3>Recent Orders</h3>
            <i class='bx bx-filter'></i>
            <i class='bx bx-search'></i>
          </div>
          <table>
            <thead>
              <tr>
                <th>User</th>
                <th>Order Date</th>
                <th>Status</th>
              </tr>
            </thead>
            <tbody>
              <tr>
                <td>
                  <img src="images/profile-1.jpg">
                  <p>John Doe</p>
                </td>
                <td>14-08-2023</td>
                <td><span class="status completed">Completed</span></td>
              </tr>
              <tr>
                <td>
                  <img src="images/profile-1.jpg">
                  <p>John Doe</p>
                </td>
                <td>14-08-2023</td>
                <td><span class="status pending">Pending</span></td>
              </tr>
              <tr>
                <td>
                  <img src="images/profile-1.jpg">
                  <p>John Doe</p>
                </td>
                <td>14-08-2023</td>
                <td><span class="status process">Processing</span></td>
              </tr>
            </tbody>
          </table>
        </div>

        <!-- Reminders -->
        <div class="reminders">
          <div class="header">
            <i class='bx bx-note'></i>
            <h3>Remiders</h3>
            <i class='bx bx-filter'></i>
            <i class='bx bx-plus'></i>
          </div>
          <ul class="task-list">
            <li class="completed">
              <div class="task-title">
                <i class='bx bx-check-circle'></i>
                <p>Start Our Meeting</p>
              </div>
              <i class='bx bx-dots-vertical-rounded'></i>
            </li>
            <li class="completed">
              <div class="task-title">
                <i class='bx bx-check-circle'></i>
                <p>Analyse Our Site</p>
              </div>
              <i class='bx bx-dots-vertical-rounded'></i>
            </li>
            <li class="not-completed">
              <div class="task-title">
                <i class='bx bx-x-circle'></i>
                <p>Play Footbal</p>
              </div>
              <i class='bx bx-dots-vertical-rounded'></i>
            </li>
          </ul>
        </div>

        <!-- End of Reminders-->

      </div>

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
  position:absolute;
  top: 25;
  /* left: 0; */
  right: 0;
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

.titlebar-button:hover {
  background: #06b139;
}
</style>
