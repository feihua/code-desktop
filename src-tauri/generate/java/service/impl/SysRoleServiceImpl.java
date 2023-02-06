package com.example.springboottpl.service.impl;

import java.util.List;
import java.util.stream.Collectors;

import org.springframework.beans.BeanUtils;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.stereotype.Service;

import com.example.springboottpl.entity.SysRole;
import com.example.springboottpl.vo.req.SysRoleReq;
import com.example.springboottpl.vo.resp.SysRoleResp;
import com.example.springboottpl.dao.SysRoleDao;
import com.example.springboottpl.service.SysRoleService;

@Service
public class SysRoleServiceImpl implements SysRoleService {

   @Autowired
   private SysRoleDao recordDao;

   @Override
   public SysRoleResp query(SysRoleReq record){

       return null;
   }

   @Override
   public List<SysRoleResp> querySysRoleList(SysRoleReq record){
        SysRole target = new SysRole();
	   //BeanUtils.copyProperties(record,target);

	   List<SysRole> query = recordDao.querySysRoleList(target);
	   return query.stream().map(x -> {
		   SysRoleResp resp = new SysRoleResp();
		   BeanUtils.copyProperties(x, resp);
		   return resp;
	   }).collect(Collectors.toList());
   }

   @Override
   public int insert(SysRoleReq record){
        SysRole target = new SysRole();

        return recordDao.insert(target);
   }

   @Override
   public int delete(int id){
        return recordDao.delete(id);
   }

   @Override
   public int update(SysRoleReq record){
        SysRole target = new SysRole();

        return recordDao.update(target);
   }

}