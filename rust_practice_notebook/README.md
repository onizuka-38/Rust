# Rust Practice Notebook

Rust 문법과 표준 라이브러리 감각을 유지하기 위한 연습 파일 모음입니다. 각 파일은 독립 실행 가능한 작은 예제로 구성되어 있습니다.

## 구성

- 파일 패턴: `practice_01_*.rs`부터 `practice_50_*.rs`
- 주제: 변수, 소유권, borrowing, enum/match, collection, iterator, error handling, trait, lifetime, thread, async, 테스트, 매크로, 알고리즘, builder pattern 등
- 대부분 파일은 `main()`과 간단한 `demo()` 함수, smoke test 형태를 가집니다.

## 실행 예시

Cargo 프로젝트가 아니라 단일 Rust 파일 모음입니다. 개별 파일을 컴파일하거나 `rustc`로 직접 실행 파일을 만들 수 있습니다.

```powershell
rustc practice_01_variables_and_mutability.rs
.\practice_01_variables_and_mutability.exe
```

테스트가 있는 파일은 다음처럼 확인할 수 있습니다.

```powershell
rustc --test practice_50_builder_pattern.rs -o practice_50_tests.exe
.\practice_50_tests.exe
```

## 메모

일부 예제는 학습용 스케치라 코드가 압축되어 있거나 `TODO` 주석이 남아 있을 수 있습니다. 목적은 완성 라이브러리보다 Rust 개념을 빠르게 재확인하는 데 있습니다.
