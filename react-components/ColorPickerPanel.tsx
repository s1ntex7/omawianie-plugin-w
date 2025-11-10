// ============================================================================
// COLOR PICKER - React Component
// ============================================================================

import React, { useState } from 'react';
import { invoke } from '@tauri-apps/api/core';
import toast from 'react-hot-toast';

interface ColorResult {
  x: number;
  y: number;
  hex: string;
  rgb: string;
  rgba: string;
  hsl: string;
  hsla: string;
}

export const ColorPickerPanel: React.FC = () => {
  const [color, setColor] = useState<ColorResult | null>(null);
  const [format, setFormat] = useState<'hex' | 'rgb' | 'rgba' | 'hsl' | 'hsla'>('hex');
  const [isPicking, setIsPicking] = useState(false);

  const pickColor = async () => {
    setIsPicking(true);

    try {
      const result = await invoke<ColorResult>('colorpicker_pick');
      setColor(result);
      toast.success(`Copied ${result[format]} to clipboard!`);
    } catch (err: any) {
      if (!err.includes('cancelled')) {
        toast.error(`Failed to pick color: ${err}`);
      }
    } finally {
      setIsPicking(false);
    }
  };

  const changeFormat = async (newFormat: typeof format) => {
    setFormat(newFormat);
    try {
      await invoke('colorpicker_set_format', { format: newFormat });
    } catch (err) {
      console.error('Failed to set format:', err);
    }
  };

  const copyValue = (value: string) => {
    navigator.clipboard.writeText(value);
    toast.success('Copied to clipboard!');
  };

  return (
    <div className="panel color-picker">
      <h2>Color Picker</h2>

      <div className="format-selector">
        <label>Default format:</label>
        <select value={format} onChange={(e) => changeFormat(e.target.value as typeof format)}>
          <option value="hex">HEX</option>
          <option value="rgb">RGB</option>
          <option value="rgba">RGBA</option>
          <option value="hsl">HSL</option>
          <option value="hsla">HSLA</option>
        </select>
      </div>

      <button onClick={pickColor} disabled={isPicking} className="pick-button">
        {isPicking ? 'Click anywhere on screen...' : 'Pick Color (Ctrl+Shift+C)'}
      </button>

      {color && (
        <div className="color-result">
          <div
            className="color-preview"
            style={{ background: color.hex }}
          />

          <div className="color-values">
            <div className="value-row" onClick={() => copyValue(color.hex)}>
              <span className="label">HEX:</span>
              <span className="value">{color.hex}</span>
            </div>

            <div className="value-row" onClick={() => copyValue(color.rgb)}>
              <span className="label">RGB:</span>
              <span className="value">{color.rgb}</span>
            </div>

            <div className="value-row" onClick={() => copyValue(color.rgba)}>
              <span className="label">RGBA:</span>
              <span className="value">{color.rgba}</span>
            </div>

            <div className="value-row" onClick={() => copyValue(color.hsl)}>
              <span className="label">HSL:</span>
              <span className="value">{color.hsl}</span>
            </div>

            <div className="value-row" onClick={() => copyValue(color.hsla)}>
              <span className="label">HSLA:</span>
              <span className="value">{color.hsla}</span>
            </div>
          </div>

          <p className="hint">Click any value to copy</p>
        </div>
      )}

      <style>{`
        .color-picker {
          max-width: 400px;
        }

        .format-selector {
          margin-bottom: 16px;
          display: flex;
          align-items: center;
          gap: 12px;
        }

        .format-selector label {
          color: #aaa;
          font-size: 14px;
        }

        .format-selector select {
          padding: 8px 12px;
          background: #2a2a2a;
          color: white;
          border: 1px solid #444;
          border-radius: 4px;
          cursor: pointer;
        }

        .pick-button {
          width: 100%;
          padding: 16px;
          background: #007bff;
          color: white;
          border: none;
          border-radius: 4px;
          cursor: pointer;
          font-size: 16px;
          transition: background 0.2s;
        }

        .pick-button:hover:not(:disabled) {
          background: #0056b3;
        }

        .pick-button:disabled {
          background: #555;
          cursor: wait;
        }

        .color-result {
          margin-top: 24px;
        }

        .color-preview {
          width: 100%;
          height: 120px;
          border-radius: 8px;
          border: 2px solid #444;
          box-shadow: 0 2px 8px rgba(0, 0, 0, 0.2);
          margin-bottom: 16px;
        }

        .color-values {
          display: flex;
          flex-direction: column;
          gap: 8px;
        }

        .value-row {
          padding: 10px 12px;
          background: #2a2a2a;
          border-radius: 4px;
          display: flex;
          justify-content: space-between;
          cursor: pointer;
          transition: background 0.2s;
        }

        .value-row:hover {
          background: #333;
        }

        .label {
          color: #888;
          font-weight: 500;
        }

        .value {
          color: white;
          font-family: 'Courier New', monospace;
        }

        .hint {
          margin-top: 12px;
          text-align: center;
          color: #666;
          font-size: 12px;
          font-style: italic;
        }
      `}</style>
    </div>
  );
};
