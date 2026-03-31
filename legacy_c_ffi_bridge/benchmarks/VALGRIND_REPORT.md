# Valgrind Memory Safety Report

실행 일시:
- 

명령:
```bash
valgrind --leak-check=full --show-leak-kinds=all \
  python -c "import legacy_c_ffi_bridge as b; print(b.matmul(2,2,[1,2,3,4],2,2,[1,0,0,1]))"
```

요약:
- definitely lost: 
- indirectly lost: 
- possibly lost: 
- still reachable: 
- invalid read/write: 

결론:
- [ ] 누수 없음
- [ ] invalid memory access 없음
- [ ] 수정 필요
