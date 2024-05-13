<template>
    <el-form :model="form" label-width="80px">
        <el-form-item label="服务器">
            <el-select v-model="value" placeholder="选择一个保存的服务器配置" style="width: 240px">
                <el-option v-for="item in cities" :key="item.value" :label="item.label" :value="item.value">
                    <span style="float: left">{{ item.label }}</span>
                    <span style="float: right;color: var(--el-text-color-secondary);font-size: 13px;">{{ item.value
                    }}</span>
                </el-option>
            </el-select>
        </el-form-item>
        <el-form-item label="IP地址">
            <el-input v-model="form.serverAddress" placeholder="IP地址..." clearable style="width: 240px" />
        </el-form-item>
        <el-form-item label="端口">
            <el-input v-model="form.port" placeholder="端口..." clearable style="width: 240px" />
        </el-form-item>
        <el-form-item label="TCP/HTTP">
            <el-radio-group v-model="form.method">
                <el-radio label="TCP" />
                <el-radio label="HTTP" />
            </el-radio-group>
        </el-form-item>
        <el-form-item label="HL7报文">
            <el-input v-model="form.message" size="small" :autosize="{ minRows: 5, maxRows: 25 }" type="textarea" />
        </el-form-item>
        <el-form-item>
            <el-button type="primary" @click="onSubmit">发送</el-button>
            <!-- <el-button>Cancel</el-button> -->
        </el-form-item>

        <el-form-item label="响应">
            <el-input v-model="response" size="small" :autosize="{ minRows: 5, maxRows: 25 }" type="textarea" />
        </el-form-item>
    </el-form>
</template>

<script setup lang="ts">
import { reactive } from 'vue'
import { invoke } from '@tauri-apps/api/tauri'

// do not use same name with ref
const form = reactive({
    serverAddress: '192.168.101.127',
    port: '206',
    method: 'HTTP',
    message: `MSH|^~\&|Varian|RadOnc|HIS|EXTCARE|20161118152743||ADT^A04|20161118131|P|2.5.1||||||UTF-8
EVN|A04|202211030800
PID|1|47898456|JG89833^^^^PPN|UID^^^^UID| Eiudo^William^A^Jr.^^^D~Mike^^^^^^M|Smith|20000101|F||1002-5^American Indian or Alaska Native~2131-1^Other|120,Baker Street^^Cleavland^OHIO^500520^USA^H^^USA~121,HansesStreet^^Cleavland^OHIO^500520^USA^P^^USA~125,Hanses Street^^Cleavland^OHIO^500520^USA^C^^USA|| (0253)2390390^^PH~9595959959^^CP^^^^^^Alltel (1)~(0253)2390390^^PH(0253)2380380^ORN~88888^BPN^BP~55555^^FX~^NET^Internet^abc@gmail.com|(020)020020020|POL^Polish| S^SINGLE|PRO^Protestant||SSN989898G5|||2135-2^Hispanic or Latino|Palo Alto|||USA^United States||USA^United States|
ROL|1|UP|AT|VarOnc001^Negrin^John^H^^^^^^^L^^DN~^^^Dr Negrin^^^^^^^D
ROL|2|UP|AT|VarOnc003^Ghosh^Shubhodeep^^^^^^^^L^^DN~101011^^^Dr Ghosh^^^^^^^D^^Doctor_12
ROL|3|UP|RP|VarOnc002^Martinez^Luis^^^^^^^^L^^DN~^^^Dr Martinez^^^^^^^D
ROL|4|UP|PP|VarOnc004^Pereira^Alberto^^^^^^^^L^^DN~^^^Dr Pereira^^^^^^^D
NK1|1|Jones^Smith^^I|FTH^Father|120,Baker Street^^Palo Alto^CA^90878^USA^^^USA|(0253)2390390^^PH~(0253)2380380^ORN~55555^^FX~^^NET^abc@gmail.com~(0253)2390390^PRN^PH~9595959959^^CP|(020)020020020
NK1|2|Jones^Merry^^I|MTH^Mother^^PRIMARY|120,Baker Street^^Palo Alto^CA^500420^USA^^^USA|(0253)2390390^^PH~(0253)2380380^ORN~55555^^FX~^^NET^abc@gmail.com~(0253)2390390^PRN^PH~9595959959^^CP|(020)020020020
NK1|3|^IPS institute|EMR^EMPLOYER^^PRIMARY|160,Baker Street^^Palo Alto^CA^90878^USA^^^USA|(0253)2390390^^PH
PV1|1|I|^1235^^ACHospital^^^OIS_ID1||||VarOnc003^Ghosh^Shubhodeep^^^^^^^^L^^DN~101011^Dr Ghosh^^^^^^^^^D^^Doctor_12|VarOnc002^Martinez^Luis^^^^^^^^L^^DN~^^^Dr Martinez^^^^^^^D|||||R|||||||||||||||||||||||||||||||20150709000000
PV2||||||||||||||||||||||||||||||||||||||A^Ambulance^^^02532333
AL1|1|DA^Drug|^Dairy product (Lactose)^^^Comments for the Allergy|SV|44 ARIA Connect Demographics Outbound Interface Specification GuideSwollen-lips~Photosensitivity~Skin Rashes/Hives|20150305
AL1|2|FA^Food|^Dairy products (lactose)|XX|Other|20161117
DG1|1||230.2^Carcinoma in situ of stomach^ICD-9-CM|Carcinoma in situ of stomach|20140110000000|W|||||||||4||||20150518000000
DG1|2||235.9^Neoplasm of uncertain behavior of other and unspecified respiratory organs^ICD-9-CM|uodatt tr|20161108000000|W|||||||||||||20161118000000
IN1|1|00N||Health Group Plan|50,John Street^^Cleanland^OHIO^500420^USA||3402929902^^PH~InFax1232^^FX~^NET^Internet^abc@gmail.com|||||20070223||1^20150225^12|SFP|William|OWN^Owner||||||||||||20141009000000|||||||222277777|10.0000||||||||||||123546
PDA|Accident|||||Y`,
})

const value = ref('')
let response = ref('') as globalThis.Ref<string>;
const cities = [
    {
        value: 'Beijing',
        label: 'Beijing',
    },
    {
        value: 'Shanghai',
        label: 'Shanghai',
    },
    {
        value: 'Nanjing',
        label: 'Nanjing',
    },
    {
        value: 'Chengdu',
        label: 'Chengdu',
    },
    {
        value: 'Shenzhen',
        label: 'Shenzhen',
    },
    {
        value: 'Guangzhou',
        label: 'Guangzhou',
    },
]

const onSubmit = () => {
    invoke('send_hl7_message', form).then((message) => response.value = message as string).catch((err) => response.value = err as string)
}
</script>

<style lang="css" >
.el-textarea__inner {
    white-space: pre;
}
</style>