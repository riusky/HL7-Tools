<template>
    <nav data-tauri-drag-region>
        <el-icon v-show="!sidebar" @click="changeSidebar" class='bx' :size="25">
            <Fold />
        </el-icon>
        <el-icon v-show="sidebar" @click="changeSidebar" class='bx' :size="25">
            <Expand />
        </el-icon>
        <div class="titlebar">
            <div class="titlebar-button">
                <el-icon @click="toggleDarkDark" v-show="!store.isDark" :size="20"
                    style="color: (store.isDark ? '#030b25' : '#DCDFE6');">
                    <Moon />
                </el-icon>
                <el-icon @click="toggleDarkDark" v-show="store.isDark" :size="20"
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
</template>

<script setup lang="ts">

import { ElMessage } from 'element-plus'
import { toggleDark } from "@/composables";
import { useCounterStore } from '@/stores/windowsize'

const sidebar = ref(false)
const store = useCounterStore()

function changeSidebar() {
    if (window.innerWidth < 768) {
        ElMessage({
            message: '窗口太小已禁用侧边栏展开.请调整窗口大小后重试!',
            type: 'warning',
        })
        sidebar.value = true
        return
    }
    const sideBar = document.querySelector('.sidebar');
    sidebar.value = !sidebar.value
    sideBar?.classList.toggle('close');
}

function toggleDarkDark() {
    toggleDark();
    store.isDark = !store.isDark
    if (store.isDark) {
        document.body.classList.add('dark');
    } else {
        document.body.classList.remove('dark');
    }
}
</script>

<style scoped></style>