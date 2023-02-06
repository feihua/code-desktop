package com.example.springboottpl.controller;

import io.swagger.annotations.Api;
import io.swagger.annotations.ApiOperation;

import java.util.List;

import javax.validation.Valid;

import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.web.bind.annotation.PostMapping;
import org.springframework.web.bind.annotation.RequestBody;
import org.springframework.web.bind.annotation.RequestMapping;
import org.springframework.web.bind.annotation.RestController;

import com.example.springboottpl.vo.req.SysMenuRoleReq;
import com.example.springboottpl.vo.resp.SysMenuRoleResp;
import com.example.springboottpl.service.SysMenuRoleService;

@Api(tags = "菜单角色关联表")
@RestController
@RequestMapping("/sys_menu_role")
public class SysMenuRoleController {

   @Autowired
   private SysMenuRoleService recordService;

   @ApiOperation("查询菜单角色关联表")
   @PostMapping("/query")
   public SysMenuRoleResp query(@RequestBody @Valid SysMenuRoleReq record){
       return recordService.query(record);
   }

   @ApiOperation("查询菜单角色关联表列表")
   @PostMapping("/querySysMenuRoleList")
   public List<SysMenuRoleResp> querySysMenuRoleList(@RequestBody @Valid SysMenuRoleReq record){
        return recordService.querySysMenuRoleList(record);
   }

   @ApiOperation("添加菜单角色关联表")
   @PostMapping("/insert")
   public int insert(@RequestBody @Valid SysMenuRoleReq record){
        return recordService.insert(record);
   }

   @ApiOperation("删除菜单角色关联表")
   @PostMapping("/delete")
   public int delete(int id){
        return recordService.delete(id);
   }

   @ApiOperation("更新菜单角色关联表")
   @PostMapping("/update")
   public int update(@RequestBody @Valid SysMenuRoleReq record){
        return recordService.update(record);
   }

}