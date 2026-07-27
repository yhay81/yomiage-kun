export const appMarkup = `
  <div class="app-window">
    <header id="titlebar" class="titlebar" data-tauri-drag-region>
      <div class="titlebar-product" data-tauri-drag-region>
        <span class="titlebar-mark" aria-hidden="true" data-tauri-drag-region>
          <i></i><i></i><i></i>
        </span>
        <span data-tauri-drag-region>読み上げくん</span>
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
          aria-label="閉じる" title="閉じる（バックグラウンドで動き続けます）">
          <svg viewBox="0 0 16 16" aria-hidden="true"><path d="m4 4 8 8m0-8-8 8" /></svg>
        </button>
      </div>
    </header>

    <main class="shell">
      <header class="app-header">
        <div class="brand-mark" aria-hidden="true">
          <span></span><span></span><span></span>
        </div>
        <div class="app-heading">
          <h1>読み上げくん</h1>
          <p>Discordのメッセージを、やさしい声で読み上げます。</p>
        </div>
        <div id="statusBadge" class="status-badge stopped">
          <i></i><span>停止中</span>
        </div>
      </header>

      <div id="notice" class="notice" role="status" aria-live="polite" hidden></div>

      <section class="dashboard">
        <article class="control-card">
          <p class="section-label">現在の状態</p>
          <div class="sound-illustration" aria-hidden="true">
            <span></span><span></span><span></span><span></span><span></span>
          </div>
          <h2 id="primaryStatusTitle">停止しています</h2>
          <p id="primaryStatusDetail" class="status-detail">
            準備ができたら、下のボタンから始められます。
          </p>
          <div class="primary-actions">
            <button id="startBot" class="primary" type="button">読み上げを始める</button>
            <button id="stopBot" class="danger" type="button" disabled>読み上げを止める</button>
          </div>
          <p class="join-guide">
            開始後、Discordのボイスチャンネルで <code>/join</code> と入力してください。
          </p>
        </article>

        <div class="side-column">
          <article class="summary-card">
            <div class="summary-heading">
              <div>
                <p class="section-label">かんたん確認</p>
                <h2>準備</h2>
              </div>
              <button id="openSettings" class="quiet-button" type="button">設定を開く</button>
            </div>
            <div class="readiness-list">
              <div class="readiness-item">
                <span class="readiness-icon">1</span>
                <div>
                  <strong>Discordとの連携</strong>
                  <p id="tokenSummary">確認しています…</p>
                </div>
              </div>
              <div id="engineSummaryRow" class="readiness-item">
                <span class="readiness-icon">2</span>
                <div>
                  <strong>読み上げる声</strong>
                  <p id="engineSummary">接続はまだ確認していません。</p>
                </div>
              </div>
            </div>
          </article>

          <article class="activity-card">
            <p class="section-label">接続状況</p>
            <dl class="metrics">
              <div><dt>ボット</dt><dd id="botName">—</dd></div>
              <div><dt>サーバー</dt><dd id="guildCount">0</dd></div>
              <div><dt>読み上げ先</dt><dd id="sessionCount">0</dd></div>
            </dl>
          </article>
        </div>
      </section>

      <footer>
        <span>トークンと音声は、このパソコンの外へ保存しません。</span>
        <div class="footer-actions">
          <button id="installUpdate" class="text-button update-button" type="button" hidden>更新する</button>
          <button id="exportDiagnostics" class="text-button" type="button">診断情報を保存</button>
          <button id="openDocs" class="text-button" type="button">使い方を見る ↗</button>
        </div>
      </footer>
    </main>

    <dialog id="settingsDialog" class="settings-dialog">
      <div class="dialog-header">
        <div>
          <p class="section-label">初回だけ設定してください</p>
          <h2>読み上げの設定</h2>
        </div>
        <button id="closeSettings" class="dialog-close" type="button" aria-label="設定を閉じる">×</button>
      </div>

      <div class="settings-grid">
        <section class="settings-section">
          <div class="settings-heading">
            <span>1</span>
            <div>
              <h3>Discordとつなぐ</h3>
              <p>専用のボットを安全に登録します。</p>
            </div>
          </div>
          <button id="openPortal" class="text-button portal-button" type="button">
            Discordの開発者ページを開く ↗
          </button>
          <label>
            ボットのトークン
            <div class="password-row">
              <input id="token" type="password" autocomplete="off" spellcheck="false"
                placeholder="コピーしたトークンを貼り付け" />
              <button id="toggleToken" class="icon-button" type="button"
                aria-label="トークンを表示">表示</button>
            </div>
          </label>
          <div class="button-row">
            <button id="saveToken" class="secondary" type="button">確認して保存</button>
            <button id="inviteBot" class="secondary" type="button" disabled>サーバーに追加</button>
            <button id="clearToken" class="quiet-button" type="button">連携を解除</button>
          </div>
          <p id="tokenState" class="hint">保存済みのトークンを確認しています…</p>
        </section>

        <section class="settings-section">
          <div class="settings-heading">
            <span>2</span>
            <div>
              <h3>読み上げる声を選ぶ</h3>
              <p>このパソコンの音声合成ソフトを使います。</p>
            </div>
          </div>
          <div class="two-columns">
            <label>
              音声合成ソフト
              <select id="provider">
                <option value="aivis_speech">AivisSpeech</option>
                <option value="voicevox">VOICEVOX</option>
              </select>
            </label>
            <label>
              声
              <select id="speakerId" disabled>
                <option value="1">接続すると声を選べます</option>
              </select>
            </label>
          </div>
          <div class="engine-help">
            <button id="openEngineSite" class="text-button" type="button">
              音声合成ソフトを入手する ↗
            </button>
          </div>
          <details class="advanced-settings">
            <summary>うまくつながらないときの設定</summary>
            <label>
              接続先
              <input id="endpoint" type="url" spellcheck="false" />
            </label>
          </details>
          <div id="engineStatus" class="engine-status idle" role="status" aria-live="polite">
            <span class="engine-status-dot" aria-hidden="true"></span>
            <div>
              <strong id="engineStatusTitle">接続を確認してください</strong>
              <p id="engineStatusDetail">音声合成ソフトを起動して接続を確認します。</p>
            </div>
          </div>
          <div class="range-grid">
            <label>速さ <output id="speedValue"></output><input id="speed" type="range" min="0.5" max="2" step="0.05" /></label>
            <label>高さ <output id="pitchValue"></output><input id="pitch" type="range" min="-0.15" max="0.15" step="0.01" /></label>
            <label>抑揚 <output id="intonationValue"></output><input id="intonation" type="range" min="0" max="2" step="0.05" /></label>
            <label>音量 <output id="volumeValue"></output><input id="volume" type="range" min="0" max="2" step="0.05" /></label>
          </div>
          <div class="button-row engine-actions">
            <button id="detectProvider" class="secondary" type="button">自動で見つける</button>
            <button id="testProvider" class="secondary" type="button">接続を確認</button>
            <button id="previewVoice" class="secondary" type="button" disabled>この声を試す</button>
            <button id="saveSettings" class="secondary" type="button">この設定を保存</button>
          </div>
        </section>
      </div>

      <div class="dialog-footer">
        <label class="checkbox">
          <input id="autostart" type="checkbox" />
          <span>パソコンへのログイン時に自動で始める</span>
        </label>
        <span>設定はこのパソコンだけに保存されます。</span>
      </div>
    </dialog>
  </div>
`;
