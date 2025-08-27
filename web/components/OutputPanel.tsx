"use client";
import React from "react";
import { AlertTriangle, Timer, CheckCircle, Terminal } from "lucide-react";

interface OutputPanelProps {
  output: string | null;
  error: string | null;
  executionTime: number | null;
  isLoading: boolean;
}

export const OutputPanel: React.FC<OutputPanelProps> = ({ output, error, executionTime, isLoading }) => {
  const hasOutput = output !== null;
  const hasError = error !== null;

  return (
    <div className="bg-white h-full flex flex-col text-gray-800">
       <div className="flex-shrink-0 px-4 py-2 border-b border-gray-200">
          <h2 className="text-lg font-semibold flex items-center text-black">
            <Terminal className="w-5 h-5 mr-2" />
            Console
          </h2>
      </div>
      <div className="flex-grow p-4 overflow-y-auto">
        {isLoading && <div className="text-gray-500">Executing...</div>}
        
        {!isLoading && (
          <>
            {executionTime !== null && (
              <div className="text-sm text-gray-500 mb-4 bg-gray-100 p-2 rounded-md flex items-center">
                <Timer className="h-4 w-4 mr-2" />
                <span>Execution Time: {executionTime} ms</span>
              </div>
            )}
            {hasError ? (
              <div className="bg-red-50 p-3 rounded-md border border-red-200">
                <div className="flex items-center text-red-700 font-bold mb-2">
                  <AlertTriangle className="h-5 w-5 mr-2" />
                  Error
                </div>
                <pre className="text-sm text-red-900 whitespace-pre-wrap">{error}</pre>
              </div>
            ) : hasOutput ? (
               <div className="bg-green-50 p-3 rounded-md border border-green-200">
                 <div className="flex items-center text-green-700 font-bold mb-2">
                    <CheckCircle className="h-5 w-5 mr-2" />
                    Success
                 </div>
                 <pre className="text-sm text-gray-800 whitespace-pre-wrap">{output || "Execution finished with no output."}</pre>
               </div>
            ) : (
                <div className="text-gray-500">
                    Your code&apos;s output will appear here.
                </div>
            )}
          </>
        )}
      </div>
    </div>
  );
};