<template>
  <div class="content">
    <el-form label-position="right" label-width="150px" :model="gerateForm"
             style="max-width: 460px;height: 460px;padding-top: 100px;">
      <el-form-item label="数据库地址">
        <el-input v-model="gerateForm.db_url" placeholder="mysql://root:123456@127.0.0.1:3306"/>
      </el-form-item>
      <el-form-item label="数据库">
        <el-input v-model="gerateForm.db_name" placeholder="user_table"/>
      </el-form-item>
      <el-form-item label="表(多个表用逗号分开)">
        <el-input v-model="gerateForm.db_table" placeholder="user_table"/>
      </el-form-item>
      <el-form-item label="表名前缀">
        <el-input v-model="gerateForm.t_prefix" placeholder="user_table"/>
      </el-form-item>
      <el-form-item label="包名">
        <el-input v-model="gerateForm.package_name" placeholder="com.test"/>
      </el-form-item>
      <el-form-item label="后端路径">
        <el-input v-model="gerateForm.path_name" placeholder="D:\workspace\demo\src\main\java\com\example"/>
      </el-form-item>
      <el-form-item label="前端路径">
        <el-input v-model="gerateForm.font_path_name" placeholder="D:\workspace\demo\src\main\java\com\example"/>
      </el-form-item>
      <el-form-item label="作者">
        <el-input v-model="gerateForm.author" placeholder="koobe"/>
      </el-form-item>
      <el-form-item>
        <el-button type="primary" @click="onSubmit">生成curd</el-button>
      </el-form-item>
    </el-form>
    <!-- <Greet></Greet> -->
  </div>
</template>

<script lang="ts" setup>
import {reactive} from 'vue'
import {invoke} from "@tauri-apps/api/tauri";
import {ElMessage} from 'element-plus'
import 'element-plus/es/components/message/style/css'
// import Greet from './components/Greet.vue';

const gerateForm = reactive({
  db_url: 'mysql://root:r-wz9wop62956dh5k9ed@rm-wz9a2yv489d123yqkdo.mysql.rds.aliyuncs.com:3306',
  db_name: 'rustdb',
  db_table: 'sys_user,sys_role_user,sys_role,sys_menu_role,sys_menu',
  package_name: 'com.example.springboottpl',
  path_name: 'D:\\workspace\\java\\spring-boot-tpl\\src\\main\\java\\com\\example\\springboottpl\\',
  t_prefix: 'sys_',
  font_path_name: 'E:\\generate\\',
  author: '刘飞华',
})
const onSubmit = () => {
  console.log('submit!', gerateForm.db_name)
  invoke("generate_code", {
    dbUrl: gerateForm.db_url,
    dbName: gerateForm.db_name,
    tableName: gerateForm.db_table,
    packageName: gerateForm.package_name,
    savePath: gerateForm.path_name,
    tPrefix: gerateForm.t_prefix,
    fontPathName: gerateForm.font_path_name,
    author: gerateForm.author
  }).then(() =>
      ElMessage({
        message: '生成curd代码成功',
        type: 'success',
        offset: 200,
      })
  );

}
</script>

<style scoped>
.content {
  width: 460px;
  height: 460px;
  margin: auto;
}
</style>
