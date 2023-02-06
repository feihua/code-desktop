package com.example.springboottpl.service;

import java.util.List;

import com.example.springboottpl.vo.req.SysMenuRoleReq;
import com.example.springboottpl.vo.resp.SysMenuRoleResp;

public interface SysMenuRoleService {

   SysMenuRoleResp query(SysMenuRoleReq record);

   List<SysMenuRoleResp> querySysMenuRoleList(SysMenuRoleReq record);

   int insert(SysMenuRoleReq record);

   int delete(int id);

   int update(SysMenuRoleReq record);

}