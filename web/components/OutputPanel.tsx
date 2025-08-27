"use client";
import React from "react";
import { AlertTriangle, Timer, CheckCircle } from "lucide-react";

interface OutputPanelProps {
  output: string | null;
  error: string | null;
  executionTime: number | null;
  isLoading: boolean;
}

export const OutputPanel: React.FC<OutputPanelProps> = ({ output, error, executionTime, isLoading }) => {
  return (
    <div className="bg-white h-full flex flex-col">
      <div className="flex-grow p-4 overflow-y-auto">
        {isLoading ? (
            <div className="text-gray-500">Executing...</div>
        ) : (
          <>
            {executionTime !== null && (
              <div className="flex items-center text-sm text-gray-500 mb-4 bg-gray-100 p-2 rounded-md">
                <Timer className="h-4 w-4 mr-2" />
                <span>Execution Time: {executionTime} ms</span>
              </div>
            )}
            {error ? (
              <div className="bg-red-50 p-3 rounded-md">
                <div className="flex items-center text-red-700 font-bold mb-2">
                  <AlertTriangle className="h-5 w-5 mr-2" />
                  Error
                </div>
                <pre className="text-sm text-red-900 whitespace-pre-wrap">{error}</pre>
              </div>
            ) : output !== null ? (
               <div className="bg-green-50 p-3 rounded-md">
                 <div className="flex items-center text-green-700 font-bold mb-2">
                    <CheckCircle className="h-5 w-5 mr-2" />
                    Success
                 </div>
                 <pre className="text-sm text-green-900 whitespace-pre-wrap">{output || "No output produced."}</pre>
               </div>
            ) : (
                <div className="text-gray-500">
                    Click &quot;Run Code&quot; to see the output here.
                </div>
            )}
          </>
        )}
      </div>
    </div>
  );
};