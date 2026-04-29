# Valgrind Memory Safety Report

레거시 C FFI 경로의 메모리 안전성을 Linux 환경에서 확인하기 위한 보고서 템플릿입니다.

## 실행 정보

- 실행 일시:
- 실행 환경:

## 명령

```bash
valgrind --leak-check=full --show-leak-kinds=all \
  python -c "import legacy_c_ffi_bridge as b; print(b.matmul(2,2,[1,2,3,4],2,2,[1,0,0,1]))"
```

## 요약

| 항목 | 결과 |
|---|---:|
| definitely lost | - |
| indirectly lost | - |
| possibly lost | - |
| still reachable | - |
| invalid read/write | - |

## 결론

- [ ] 누수 없음
- [ ] invalid memory access 없음
- [ ] 수정 필요
