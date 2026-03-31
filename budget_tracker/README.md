# budget_tracker

실무에서 바로 쓰기 좋은 CLI 가계부 프로젝트입니다.

## 기능
- 수입/지출 내역 추가
- 월별/카테고리별 내역 조회
- 월별 수입/지출/순이익 요약
- 잘못 입력한 내역 삭제
- JSON 파일(`transactions.json`) 기반 로컬 영속 저장

## 실행 예시
```bash
cargo run -- add expense --amount 12800 --category lunch --memo "team lunch"
cargo run -- add income --amount 3000000 --category salary --date 2026-03-25
cargo run -- list --month 2026-03
cargo run -- summary --month 2026-03
cargo run -- delete 2
```

## 데이터 파일
기본 저장 파일은 현재 실행 디렉터리의 `transactions.json` 입니다.
다른 파일을 쓰려면 `--file` 옵션을 사용하세요.

```bash
cargo run -- --file data/prod.json summary --month 2026-03
```
