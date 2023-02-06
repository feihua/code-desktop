package com.example.springboottpl.service;

import java.util.List;

import com.example.springboottpl.vo.req.SysRoleUserReq;
import com.example.springboottpl.vo.resp.SysRoleUserResp;

public interface SysRoleUserService {

   SysRoleUserResp query(SysRoleUserReq record);

   List<SysRoleUserResp> querySysRoleUserList(SysRoleUserReq record);

   int insert(SysRoleUserReq record);

   int delete(int id);

   int update(SysRoleUserReq record);

}