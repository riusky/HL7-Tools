<template>
  <el-tabs v-model="activeName" class="demo-tabs" @tab-click="handleClick">
    <el-tab-pane label="发送工具" name="first">

        <HL7SendTool/>
    </el-tab-pane>
    <el-tab-pane label="增加配置" name="second">增加配置</el-tab-pane>
    <el-tab-pane label="测试脚本" name="third">测试脚本</el-tab-pane>
    <el-tab-pane label="其他" name="fourth">其他</el-tab-pane>
  </el-tabs>
</template>

<script lang="ts" setup>
import { ref } from 'vue'
import type { TabsPaneContext } from 'element-plus'
import HL7SendTool from '@/components/hl7/HL7SendTool.vue'

const activeName = ref('first')

const handleClick = (tab: TabsPaneContext, event: Event) => {
  console.log(tab, event)
}
</script>

<style lang="scss" scoped>

.demo-tabs > .el-tabs__content {
  padding: 32px;
  color: #6b778c;
  font-size: 32px;
  font-weight: 600;
}
</style>