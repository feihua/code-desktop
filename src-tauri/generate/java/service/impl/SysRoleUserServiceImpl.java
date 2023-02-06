package com.example.springboottpl.service.impl;

import java.util.List;
import java.util.stream.Collectors;

import org.springframework.beans.BeanUtils;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.stereotype.Service;

import com.example.springboottpl.entity.SysRoleUser;
import com.example.springboottpl.vo.req.SysRoleUserReq;
import com.example.springboottpl.vo.resp.SysRoleUserResp;
import com.example.springboottpl.dao.SysRoleUserDao;
import com.example.springboottpl.service.SysRoleUserService;

@Service
public class SysRoleUserServiceImpl implements SysRoleUserService {

   @Autowired
   private SysRoleUserDao recordDao;

   @Override
   public SysRoleUserResp query(SysRoleUserReq record){

       return null;
   }

   @Override
   public List<SysRoleUserResp> querySysRoleUserList(SysRoleUserReq record){
        SysRoleUser target = new SysRoleUser();
	   //BeanUtils.copyProperties(record,target);

	   List<SysRoleUser> query = recordDao.querySysRoleUserList(target);
	   return query.stream().map(x -> {
		   SysRoleUserResp resp = new SysRoleUserResp();
		   BeanUtils.copyProperties(x, resp);
		   return resp;
	   }).collect(Collectors.toList());
   }

   @Override
   public int insert(SysRoleUserReq record){
        SysRoleUser target = new SysRoleUser();

        return recordDao.insert(target);
   }

   @Override
   public int delete(int id){
        return recordDao.delete(id);
   }

   @Override
   public int update(SysRoleUserReq record){
        SysRoleUser target = new SysRoleUser();

        return recordDao.update(target);
   }

}