# Distillery

> *생각을 증류하여 지식으로 숙성시키다.*

---

```
  몰팅         몰트 하우스        스틸          대기열          캐스크
  ─────       ───────────      ──────       ──────────      ──────
 [ 생각 ] ──→ [  보관  ] ──→ [  선별  ] ──→ [  증류  ] ──→ [ 지식 ]
               ↑                   │
               └───── 드로우백 ─────┘
```

Distillery는 위스키 증류소의 과정을 빌려 **생각을 정제하는 데스크톱 앱**입니다.

흩어진 결정, 문제, 인사이트, 질문을 **몰트**로 빚고,
**스틸**에 올려 선별한 뒤,
서버가 매일 정오와 자정에 자동으로 증류하여
**캐스크**(지식 그래프)에서 숙성시킵니다.

---

## 증류 과정

### 1. 몰팅 (Malting)

생각을 몰트로 만듭니다. 네 가지 유형 중 하나를 선택하세요.

| 유형 | 설명 |
|------|------|
| **결정** | 확정된 의사결정 |
| **문제** | 해결이 필요한 이슈 |
| **인사이트** | 발견한 통찰 |
| **질문** | 논의가 필요한 질문 |

### 2. 몰트 하우스 (Malt House)

만들어진 몰트를 보관하는 창고. 수정, 삭제, 검색이 가능합니다.

### 3. 스틸 (Still)

증류할 몰트를 선별하는 단계. 준비가 되면 증류 대기열로 보냅니다.

### 4. 드로우백 (Draw Back)

대기열에서 몰트를 빼냅니다. 캐스크에 들어간 몰트는 되돌릴 수 없습니다.

---

## 기술 스택

```
Frontend    Svelte 5 · SvelteKit · TypeScript · Tailwind CSS · DaisyUI
Backend     Rust · SQLite (FTS5) · reqwest
Desktop     Tauri 2
```

---

## 시작하기

### 준비물

- [Bun](https://bun.sh) (또는 Node.js)
- [Rust toolchain](https://rustup.rs)
- [Tauri 2 prerequisites](https://v2.tauri.app/start/prerequisites/)

### 설치

```bash
# 의존성 설치
bun install

# 환경 변수 설정
cp src-tauri/.env.example src-tauri/.env
# src-tauri/.env 에서 API_BASE_URL 설정
```

### 개발

```bash
# Tauri 개발 서버 실행
bun run tauri dev
```

### 빌드

```bash
# 프로덕션 빌드
bun run tauri build
```

---

## 프로젝트 구조

```
src/
├── routes/              # 페이지 (몰팅, 몰트하우스, 스틸, 드로우백, 설정, 도움말)
├── lib/
│   ├── components/      # Svelte 컴포넌트
│   ├── stores/          # 상태 관리 (auth, malts, settings)
│   ├── i18n/            # 다국어 (한국어, English)
│   └── types.ts         # 타입 정의
src-tauri/
├── src/
│   ├── commands.rs      # Tauri IPC 명령
│   ├── db.rs            # SQLite 데이터베이스
│   ├── auth.rs          # OTP 인증
│   ├── api.rs           # 서버 API 통신
│   └── models.rs        # 데이터 모델
```

---

<sub>pitch black, amber-lit.</sub>
