package com.example.springboottpl.service;

import java.util.List;

import com.example.springboottpl.vo.req.SysRoleReq;
import com.example.springboottpl.vo.resp.SysRoleResp;

public interface SysRoleService {

   SysRoleResp query(SysRoleReq record);

   List<SysRoleResp> querySysRoleList(SysRoleReq record);

   int insert(SysRoleReq record);

   int delete(int id);

   int update(SysRoleReq record);

}