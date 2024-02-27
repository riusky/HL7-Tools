<template>
    <div class="sidebar">
        <a href="#" class="logo">
            <i class='bx bx-code-alt'></i>
            <div class="logo-name"><span>WIN</span>GED</div>
        </a>
        <ul class="side-menu">
            <li @click="to_url('a')"><a href="#"><el-icon class='bx bxs-dashboard'>
                        <Menu />
                    </el-icon>首页</a></li>
            <li><a href="#"><el-icon class='bx bx-store-alt'>
                        <Document />
                    </el-icon>代码</a></li>
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
</template>

<script setup lang="ts">
// import { to_url } from "@/composables";


function to_url(routerName: string) {
 
    console.log("routerName",routerName)
    const proxy = getCurrentInstance()?.proxy;
    console.log("proxy",proxy)
    if (proxy == null) {
        return
    }
    proxy.$router.push({ name: routerName })
 
}




</script>

<style scoped></style>