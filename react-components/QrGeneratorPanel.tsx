// ============================================================================
// QR GENERATOR - React Component
// ============================================================================

import React, { useState } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { save } from '@tauri-apps/plugin-dialog';
import toast from 'react-hot-toast';

export const QrGeneratorPanel: React.FC = () => {
  const [text, setText] = useState('');
  const [qrDataUrl, setQrDataUrl] = useState<string | null>(null);
  const [isGenerating, setIsGenerating] = useState(false);

  const generate = async () => {
    if (!text.trim()) {
      toast.error('Please enter text or URL');
      return;
    }

    setIsGenerating(true);

    try {
      const dataUrl = await invoke<string>('qr_generate', { text });
      setQrDataUrl(dataUrl);
      toast.success('QR code copied to clipboard!');
    } catch (err) {
      toast.error(`Failed to generate: ${err}`);
    } finally {
      setIsGenerating(false);
    }
  };

  const saveToFile = async () => {
    if (!qrDataUrl) return;

    try {
      const filePath = await save({
        filters: [{ name: 'PNG Image', extensions: ['png'] }],
        defaultPath: 'qrcode.png',
      });

      if (filePath) {
        await invoke('qr_save', { text, filePath });
        toast.success(`Saved to ${filePath}`);
      }
    } catch (err) {
      toast.error(`Failed to save: ${err}`);
    }
  };

  return (
    <div className="panel qr-generator">
      <h2>QR Code Generator</h2>

      <div className="input-section">
        <textarea
          value={text}
          onChange={(e) => setText(e.target.value)}
          placeholder="Enter text or URL..."
          rows={4}
        />

        <button onClick={generate} disabled={isGenerating || !text.trim()}>
          {isGenerating ? 'Generating...' : 'Generate QR Code'}
        </button>
      </div>

      {qrDataUrl && (
        <div className="qr-result">
          <img src={qrDataUrl} alt="QR Code" />
          <button onClick={saveToFile}>Save to File</button>
        </div>
      )}

      <style>{`
        .qr-generator {
          max-width: 500px;
        }

        .input-section {
          display: flex;
          flex-direction: column;
          gap: 12px;
        }

        textarea {
          padding: 12px;
          border: 1px solid #444;
          border-radius: 4px;
          background: #2a2a2a;
          color: white;
          font-family: inherit;
          font-size: 14px;
          resize: vertical;
        }

        button {
          padding: 12px 24px;
          background: #007bff;
          color: white;
          border: none;
          border-radius: 4px;
          cursor: pointer;
          font-size: 14px;
          transition: background 0.2s;
        }

        button:hover:not(:disabled) {
          background: #0056b3;
        }

        button:disabled {
          background: #555;
          cursor: not-allowed;
        }

        .qr-result {
          margin-top: 20px;
          text-align: center;
        }

        .qr-result img {
          max-width: 300px;
          border: 1px solid #444;
          border-radius: 4px;
          padding: 16px;
          background: white;
        }

        .qr-result button {
          margin-top: 12px;
          background: #28a745;
        }

        .qr-result button:hover {
          background: #218838;
        }
      `}</style>
    </div>
  );
};
