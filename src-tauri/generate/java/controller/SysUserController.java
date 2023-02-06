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

import com.example.springboottpl.vo.req.SysUserReq;
import com.example.springboottpl.vo.resp.SysUserResp;
import com.example.springboottpl.service.SysUserService;

@Api(tags = "后台用户信息")
@RestController
@RequestMapping("/sys_user")
public class SysUserController {

   @Autowired
   private SysUserService recordService;

   @ApiOperation("查询后台用户信息")
   @PostMapping("/query")
   public SysUserResp query(@RequestBody @Valid SysUserReq record){
       return recordService.query(record);
   }

   @ApiOperation("查询后台用户信息列表")
   @PostMapping("/querySysUserList")
   public List<SysUserResp> querySysUserList(@RequestBody @Valid SysUserReq record){
        return recordService.querySysUserList(record);
   }

   @ApiOperation("添加后台用户信息")
   @PostMapping("/insert")
   public int insert(@RequestBody @Valid SysUserReq record){
        return recordService.insert(record);
   }

   @ApiOperation("删除后台用户信息")
   @PostMapping("/delete")
   public int delete(int id){
        return recordService.delete(id);
   }

   @ApiOperation("更新后台用户信息")
   @PostMapping("/update")
   public int update(@RequestBody @Valid SysUserReq record){
        return recordService.update(record);
   }

}