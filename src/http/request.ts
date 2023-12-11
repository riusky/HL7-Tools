//  配置封装axios 请求
//  引入axios 
import axios from "axios";
//引入element plus
import { ElMessage } from "element-plus";
 
// 跨域请求，允许保存cookie
axios.defaults.withCredentials = true;
 
const instance = axios.create({
    // 配置公共请求地址
    baseURL: '/api',
    // 配置请求时长 (当请求超过该时间,则自动中断请求)
    timeout: 50000,
    // 配置请求头(只针对post 请求有效)
    headers: { 'Content-Type': 'application/json' }
 
});
 
// 添加一个请求拦截器
//3.请求拦截    登陆放token的地方
instance.interceptors.request.use(config => {
    // config.headers['Authorization'] =getCookie('token')
    config.headers['Access-Control-Allow-Origin'] = '*'
    return config
})
// 添加一个响应拦截器
instance.interceptors.response.use(
    (res) => {
        if (res.status === 200 || res.data.code == 200) { 
            return res.data
        } else {
            ElMessage.error('请求失败')
        }
    },
    (err) => {
        //全局错误提示
        ElMessage.error(err.message)
        // if (err.response.status === 404) {
        //根据需求判断不同错误信息返回不同信息
        // }
        return Promise.reject(err);  //将错误消息挂到promise的失败函数上
    },
 
)
export default instance