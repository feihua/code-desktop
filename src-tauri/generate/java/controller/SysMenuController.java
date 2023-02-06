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

import com.example.springboottpl.vo.req.SysMenuReq;
import com.example.springboottpl.vo.resp.SysMenuResp;
import com.example.springboottpl.service.SysMenuService;

@Api(tags = "菜单信息")
@RestController
@RequestMapping("/sys_menu")
public class SysMenuController {

   @Autowired
   private SysMenuService recordService;

   @ApiOperation("查询菜单信息")
   @PostMapping("/query")
   public SysMenuResp query(@RequestBody @Valid SysMenuReq record){
       return recordService.query(record);
   }

   @ApiOperation("查询菜单信息列表")
   @PostMapping("/querySysMenuList")
   public List<SysMenuResp> querySysMenuList(@RequestBody @Valid SysMenuReq record){
        return recordService.querySysMenuList(record);
   }

   @ApiOperation("添加菜单信息")
   @PostMapping("/insert")
   public int insert(@RequestBody @Valid SysMenuReq record){
        return recordService.insert(record);
   }

   @ApiOperation("删除菜单信息")
   @PostMapping("/delete")
   public int delete(int id){
        return recordService.delete(id);
   }

   @ApiOperation("更新菜单信息")
   @PostMapping("/update")
   public int update(@RequestBody @Valid SysMenuReq record){
        return recordService.update(record);
   }

}