# budget_tracker

로컬 JSON 파일에 수입과 지출을 저장하는 CLI 가계부입니다. 별도 서버나 데이터베이스 없이 현재 실행 디렉터리의 `transactions.json`을 기본 저장소로 사용합니다.

## 기능

- 수입/지출 내역 추가
- 월별, 카테고리별 내역 조회
- 월별 수입/지출/순이익 요약
- 지출 카테고리 상위 항목 출력
- 잘못 입력한 거래 삭제
- `--file` 옵션으로 저장 파일 변경

## 실행 예시

```powershell
cd budget_tracker
cargo run -- add expense --amount 12800 --category lunch --memo "team lunch"
cargo run -- add income --amount 3000000 --category salary --date 2026-03-25
cargo run -- list --month 2026-03
cargo run -- summary --month 2026-03
cargo run -- delete 2
```

다른 데이터 파일을 사용하려면 전역 옵션 `--file`을 먼저 전달합니다.

```powershell
cargo run -- --file data/prod.json summary --month 2026-03
```

## 데이터 형식

저장 파일은 pretty JSON으로 기록됩니다. 각 거래는 `id`, `kind`, `date`, `category`, `amount`, `memo`, `created_at` 필드를 가집니다.

## 검증

```powershell
cargo check
cargo run -- --help
```
