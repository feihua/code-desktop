package com.example.springboottpl.service;

import java.util.List;

import com.example.springboottpl.vo.req.SysMenuReq;
import com.example.springboottpl.vo.resp.SysMenuResp;

public interface SysMenuService {

   SysMenuResp query(SysMenuReq record);

   List<SysMenuResp> querySysMenuList(SysMenuReq record);

   int insert(SysMenuReq record);

   int delete(int id);

   int update(SysMenuReq record);

}