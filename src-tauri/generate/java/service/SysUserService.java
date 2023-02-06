package com.example.springboottpl.service;

import java.util.List;

import com.example.springboottpl.vo.req.SysUserReq;
import com.example.springboottpl.vo.resp.SysUserResp;

public interface SysUserService {

   SysUserResp query(SysUserReq record);

   List<SysUserResp> querySysUserList(SysUserReq record);

   int insert(SysUserReq record);

   int delete(int id);

   int update(SysUserReq record);

}