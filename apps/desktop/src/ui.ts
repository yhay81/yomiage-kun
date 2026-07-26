export const appMarkup = `
  <div class="app-window">
    <header id="titlebar" class="titlebar" data-tauri-drag-region>
      <div class="titlebar-product" data-tauri-drag-region>
        <span class="titlebar-mark" aria-hidden="true" data-tauri-drag-region>
          <i></i><i></i><i></i>
        </span>
        <span data-tauri-drag-region>Yomiage-kun</span>
        <small data-tauri-drag-region>LOCAL</small>
      </div>
      <div class="window-controls">
        <button id="windowMinimize" class="window-control" type="button"
          aria-label="最小化" title="最小化">
          <svg viewBox="0 0 16 16" aria-hidden="true"><path d="M3 8.5h10" /></svg>
        </button>
        <button id="windowMaximize" class="window-control" type="button"
          aria-label="最大化" title="最大化">
          <svg viewBox="0 0 16 16" aria-hidden="true"><rect x="3.5" y="3.5" width="9" height="9" rx="1" /></svg>
        </button>
        <button id="windowClose" class="window-control close" type="button"
          aria-label="閉じる" title="閉じる（バックグラウンドで継続）">
          <svg viewBox="0 0 16 16" aria-hidden="true"><path d="m4 4 8 8m0-8-8 8" /></svg>
        </button>
      </div>
    </header>

    <div class="app-scroll">
      <main class="shell">
    <header class="hero">
      <div class="brand-mark" aria-hidden="true">
        <span></span><span></span><span></span>
      </div>
      <div>
        <p class="eyebrow">LOCAL-FIRST DISCORD TTS</p>
        <h1>Yomiage-kun</h1>
        <p class="lead">Discordの日本語チャットを、あなたのPCから読み上げます。</p>
      </div>
      <div id="statusBadge" class="status-badge stopped">
        <i></i><span>停止中</span>
      </div>
    </header>

    <div id="notice" class="notice" role="status" aria-live="polite" hidden></div>

    <section class="grid">
      <article class="card setup-card">
        <div class="card-heading">
          <span class="step">01</span>
          <div>
            <h2>Discord Bot</h2>
            <p>トークンはOSの安全な資格情報ストアだけに保存されます。</p>
          </div>
        </div>
        <button id="openPortal" class="text-button" type="button">
          Discord Developer Portalを開く ↗
        </button>
        <label>
          Botトークン
          <div class="password-row">
            <input id="token" type="password" autocomplete="off" spellcheck="false"
              placeholder="Developer Portalから貼り付け" />
            <button id="toggleToken" class="icon-button" type="button" aria-label="トークンを表示">表示</button>
          </div>
        </label>
        <div class="button-row">
          <button id="saveToken" class="secondary" type="button">検証して保存</button>
          <button id="inviteBot" class="secondary" type="button" disabled>サーバーへ追加</button>
        </div>
        <p id="tokenState" class="hint">保存済みトークンを確認しています…</p>
      </article>

      <article class="card">
        <div class="card-heading">
          <span class="step">02</span>
          <div>
            <h2>音声エンジン</h2>
            <p>AivisSpeechまたはVOICEVOXをローカルで使用します。</p>
          </div>
        </div>
        <div class="two-columns">
          <label>
            プロバイダー
            <select id="provider">
              <option value="aivis_speech">AivisSpeech</option>
              <option value="voicevox">VOICEVOX</option>
            </select>
          </label>
          <label>
            話者ID
            <input id="speakerId" type="number" min="0" step="1" />
          </label>
        </div>
        <label>
          エンドポイント
          <input id="endpoint" type="url" spellcheck="false" />
        </label>
        <div id="engineStatus" class="engine-status idle" role="status" aria-live="polite">
          <span class="engine-status-dot" aria-hidden="true"></span>
          <div>
            <strong id="engineStatusTitle">接続を確認してください</strong>
            <p id="engineStatusDetail">音声エンジンを起動して「接続テスト」を押します。</p>
          </div>
        </div>
        <div class="range-grid">
          <label>速度 <output id="speedValue"></output><input id="speed" type="range" min="0.5" max="2" step="0.05" /></label>
          <label>抑揚 <output id="intonationValue"></output><input id="intonation" type="range" min="0" max="2" step="0.05" /></label>
          <label>音量 <output id="volumeValue"></output><input id="volume" type="range" min="0" max="2" step="0.05" /></label>
        </div>
        <div class="button-row">
          <button id="testProvider" class="secondary" type="button">接続テスト</button>
          <button id="saveSettings" class="secondary" type="button">設定を保存</button>
        </div>
      </article>

      <article class="card run-card">
        <div class="card-heading">
          <span class="step">03</span>
          <div>
            <h2>読み上げを開始</h2>
            <p>Bot起動後、Discordで <code>/join</code> を実行してください。</p>
          </div>
        </div>
        <dl class="metrics">
          <div><dt>Bot</dt><dd id="botName">—</dd></div>
          <div><dt>参加サーバー</dt><dd id="guildCount">0</dd></div>
          <div><dt>読み上げ接続</dt><dd id="sessionCount">0</dd></div>
        </dl>
        <label class="checkbox">
          <input id="autostart" type="checkbox" />
          <span>ログイン時に自動起動する</span>
        </label>
        <div class="button-row primary-actions">
          <button id="startBot" class="primary" type="button">Botを開始</button>
          <button id="stopBot" class="danger" type="button" disabled>停止</button>
        </div>
      </article>
    </section>

    <footer>
      <span>音声とDiscordトークンは、このPCの外へ保存しません。</span>
      <button id="openDocs" class="text-button" type="button">セットアップガイド ↗</button>
    </footer>
      </main>
    </div>
  </div>
`;
