package com.example.springboottpl.service.impl;

import java.util.List;
import java.util.stream.Collectors;

import org.springframework.beans.BeanUtils;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.stereotype.Service;

import com.example.springboottpl.entity.SysUser;
import com.example.springboottpl.vo.req.SysUserReq;
import com.example.springboottpl.vo.resp.SysUserResp;
import com.example.springboottpl.dao.SysUserDao;
import com.example.springboottpl.service.SysUserService;

@Service
public class SysUserServiceImpl implements SysUserService {

   @Autowired
   private SysUserDao recordDao;

   @Override
   public SysUserResp query(SysUserReq record){

       return null;
   }

   @Override
   public List<SysUserResp> querySysUserList(SysUserReq record){
        SysUser target = new SysUser();
	   //BeanUtils.copyProperties(record,target);

	   List<SysUser> query = recordDao.querySysUserList(target);
	   return query.stream().map(x -> {
		   SysUserResp resp = new SysUserResp();
		   BeanUtils.copyProperties(x, resp);
		   return resp;
	   }).collect(Collectors.toList());
   }

   @Override
   public int insert(SysUserReq record){
        SysUser target = new SysUser();

        return recordDao.insert(target);
   }

   @Override
   public int delete(int id){
        return recordDao.delete(id);
   }

   @Override
   public int update(SysUserReq record){
        SysUser target = new SysUser();

        return recordDao.update(target);
   }

}