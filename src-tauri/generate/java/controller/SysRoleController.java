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

import com.example.springboottpl.vo.req.SysRoleReq;
import com.example.springboottpl.vo.resp.SysRoleResp;
import com.example.springboottpl.service.SysRoleService;

@Api(tags = "角色信息")
@RestController
@RequestMapping("/sys_role")
public class SysRoleController {

   @Autowired
   private SysRoleService recordService;

   @ApiOperation("查询角色信息")
   @PostMapping("/query")
   public SysRoleResp query(@RequestBody @Valid SysRoleReq record){
       return recordService.query(record);
   }

   @ApiOperation("查询角色信息列表")
   @PostMapping("/querySysRoleList")
   public List<SysRoleResp> querySysRoleList(@RequestBody @Valid SysRoleReq record){
        return recordService.querySysRoleList(record);
   }

   @ApiOperation("添加角色信息")
   @PostMapping("/insert")
   public int insert(@RequestBody @Valid SysRoleReq record){
        return recordService.insert(record);
   }

   @ApiOperation("删除角色信息")
   @PostMapping("/delete")
   public int delete(int id){
        return recordService.delete(id);
   }

   @ApiOperation("更新角色信息")
   @PostMapping("/update")
   public int update(@RequestBody @Valid SysRoleReq record){
        return recordService.update(record);
   }

}