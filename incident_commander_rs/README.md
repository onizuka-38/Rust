# Incident Commander RS

Rust로 작성한 기업형 AI 장애 분석 서비스입니다. 알림, 배포 이벤트, 로그를 입력받아 Rust 기반 탐지 요약과 LLM 기반 incident report를 생성합니다.

이 프로젝트는 민감한 운영 로그를 외부 SaaS LLM으로 보내기 어려운 회사를 가정합니다. 그래서 로컬/사내 추론 백엔드를 교체 가능한 구조로 지원합니다.

- `mock`: 로컬 개발과 CI를 위한 deterministic 응답
- `ollama`: 로컬 모델 서버의 `/api/generate` 사용
- `openai-compatible`: vLLM, LocalAI, LiteLLM 등 `/v1/chat/completions` 호환 endpoint 사용

## 포트폴리오 포인트

단순 챗봇이 아니라 실제 운영 조직의 장애 대응 흐름을 모델링합니다.

- incident alert, 서비스 로그, 배포 이벤트 수집
- 영향 서비스와 대표 에러 탐지
- deterministic severity와 risk score 계산
- alert, deployment, log를 합친 timeline 생성
- 로그/알림 증거 기반 runbook 후보 매칭
- 최근 배포와 에러 증가 징후 대조
- root-cause hypothesis와 next action 생성
- Markdown incident report 생성
- 로컬, CI, 온프레미스 배포에 맞게 LLM provider 교체 가능

## 아키텍처

```text
Client / Ops Tool
  -> Axum API
  -> Detection Engine
  -> LLM Provider Trait
       -> Mock
       -> Ollama /api/generate
       -> vLLM OpenAI-compatible /v1/chat/completions
  -> Incident Report
```

## Mock Provider로 실행

mock provider는 로컬 모델이 없어도 동작합니다.

```powershell
cd incident_commander_rs
cargo run -- --provider mock --listen 127.0.0.1:8088
```

샘플 incident 요청:

```powershell
Invoke-RestMethod `
  -Method Post `
  -Uri http://127.0.0.1:8088/api/incidents/analyze `
  -ContentType "application/json" `
  -InFile examples/sample_incident.json
```

## Ollama로 실행

```powershell
cargo run -- `
  --provider ollama `
  --llm-base-url http://localhost:11434 `
  --llm-model llama3.1
```

## vLLM / OpenAI-Compatible Endpoint로 실행

vLLM 또는 사내 gateway가 OpenAI-compatible API를 노출한다고 가정합니다.

```powershell
cargo run -- `
  --provider openai-compatible `
  --llm-base-url http://gpu-inference.internal:8000 `
  --llm-model meta-llama/Llama-3.1-8B-Instruct
```

gateway token이 필요하면 환경 변수를 사용합니다.

```powershell
$env:LLM_API_KEY = "internal-token"
cargo run -- --provider openai-compatible
```

## API

### `GET /healthz`

```json
{"status":"ok"}
```

### `POST /api/incidents/analyze`

입력 형태:

```json
{
  "title": "checkout error rate spike",
  "alerts": [],
  "deployments": [],
  "logs": []
}
```

응답 필드:

- `detection`: Rust 규칙 기반 탐지 요약
- `detection.severity`: `low`, `medium`, `high`, `critical`
- `detection.risk_score`: 0부터 100까지의 deterministic score
- `detection.timeline`: alert, deployment, log를 합친 timeline
- `detection.runbook_matches`: 로그와 알림에서 매칭된 deterministic runbook 후보
- `ai_summary`: LLM이 생성한 분석
- `recommended_actions`: deterministic 대응 액션 힌트
- `markdown`: 완성된 incident report 본문

잘못된 요청은 `400 Bad Request`를 반환합니다. 빈 제목, 빈 incident, 빈 service/timestamp/level/message 같은 필수 필드를 거부합니다.

```powershell
Invoke-RestMethod `
  -Method Post `
  -Uri http://127.0.0.1:8088/api/incidents/analyze `
  -ContentType "application/json" `
  -InFile examples/empty_invalid.json
```

## Docker

Mock provider:

```powershell
docker build -t incident-commander-rs .
docker run --rm -p 8088:8088 incident-commander-rs
```

Ollama compose:

```powershell
docker compose -f docker-compose.ollama.yml up --build
```

## 점검

```powershell
.\scripts\check.ps1
```

Docker 이미지 빌드까지 포함:

```powershell
.\scripts\check.ps1 -WithDocker
```

## 구현 하이라이트

- `axum`과 `tokio` 기반 Rust async HTTP 서비스
- 기업형 AI 추론 백엔드를 위한 provider abstraction
- CI와 오프라인 데모를 위한 mock LLM
- Ollama와 vLLM/OpenAI-compatible client
- deterministic validation, severity scoring, timeline extraction
- LLM 생성 전 deterministic runbook matching
- 민감 로그를 외부 API로 보내지 않는 사내 배포 친화 구조
- SRE incident response 도메인에 맞춘 prompt 설계
