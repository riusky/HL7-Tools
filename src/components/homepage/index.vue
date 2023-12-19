<template>
  <div>
    <div class="header">
      <div class="left">
        <h1>开发工具</h1>
        <ul class="breadcrumb">
          <li><a href="#">
              有道无术,术尚可求。
            </a></li>
        </ul>
      </div>
      <a href="#" class="report">
        <i class='bx bx-cloud-download'></i>
        <span>打开新窗口</span>
      </a>
    </div>
    <!-- Insights -->
    <ul class="insights">
      <li>
        <i class='bx bx-calendar-check'></i>
        <span class="info">
          <h3>
            hl7_tools
          </h3>
          <p>hl7消息发送</p>
        </span>
      </li>
      <li><i class='bx bx-show-alt'></i>
        <span class="info">
          <h3>
            DICOM
          </h3>
          <p>模拟器</p>
        </span>
      </li>
      <li><i class='bx bx-line-chart'></i>
        <span class="info">
          <h3>
            AUTO CODE
          </h3>
          <p>Ob代码辅助</p>
        </span>
      </li>
      <li><i class='bx bx-dollar-circle'></i>
        <span class="info">
          <h3>
            MORE
          </h3>
          <p>进行中...</p>
        </span>
      </li>
    </ul>
    <!-- End of Insights -->

    <div class="bottom-data">
      <div class="orders">
        <div class="header">
          <i class='bx bx-receipt'></i>
          <h3>开发计划</h3>
          <i class='bx bx-filter'></i>
          <i class='bx bx-search'></i>
        </div>
        <table>
          <thead>
            <tr>
              <th>项目</th>
              <th>时间</th>
              <th>状态</th>
            </tr>
          </thead>
          <tbody>
            <tr>
              <td>
                <p>界面搭建</p>
              </td>
              <td>10-12-2023</td>
              <td><span class="status completed">Completed</span></td>
            </tr>
            <tr>
              <td>
                <p>HL7发送工具</p>
              </td>
              <td>15-12-2023</td>
              <td><span class="status process">Processing</span></td>
            </tr>
            <tr>
              <td>
                <p>DICOM</p>
              </td>
              <td>16-12-2023</td>
              <td><span class="status pending">Pending</span></td>
            </tr>
            <tr>
              <td>
                <p>代码生成</p>
              </td>
              <td>23-12-2023</td>
              <td><span class="status process">Processing</span></td>
            </tr>
          </tbody>
        </table>
      </div>

      <!-- Reminders -->
      <div class="reminders">
        <div class="header">
          <i class='bx bx-note'></i>
          <h3>备忘</h3>
          <i class='bx bx-filter'></i>
          <i class='bx bx-plus'></i>
        </div>
        <ul class="task-list">
          <li class="completed">
            <div class="task-title">
              <i class='bx bx-check-circle'></i>
              <p>使用SQLite完成本机的数据持久化</p>
            </div>
            <i class='bx bx-dots-vertical-rounded'></i>
          </li>
          <li class="completed">
            <div class="task-title">
              <i class='bx bx-check-circle'></i>
              <p>DICOM使用rust库</p>
            </div>
            <i class='bx bx-dots-vertical-rounded'></i>
          </li>
          <li class="not-completed">
            <div class="task-title">
              <i class='bx bx-x-circle'></i>
              <p>软件自动升级使用内网穿透完成</p>
            </div>
            <i class='bx bx-dots-vertical-rounded'></i>
          </li>
        </ul>
      </div>
    </div>
  </div>
</template>

<script lang="ts" setup>




</script>

<style lang="scss" scoped></style>