"use client";
import React from "react";
import Link from "next/link";

export default function AboutPage() {
  return (
    <div className="min-h-screen bg-white">
      <div className="max-w-4xl mx-auto px-6 py-16">
        

        <header className="mb-16">
          <div className="text-center">
            <h1 className="text-5xl font-bold text-black mb-4">About Code.Connect</h1>
            <p className="text-xl text-gray-600">Empowering developers with modern compilation tools</p>
            <div className="w-24 h-1 bg-black mx-auto mt-6"></div>
          </div>
        </header>

        <section className="mb-16">
          <div className="text-center mb-12">
            <h2 className="text-3xl font-bold text-black mb-4">Our Mission</h2>
            <div className="w-16 h-0.5 bg-black mx-auto"></div>
          </div>
          <div className="bg-gray-50 border border-gray-200 rounded-lg p-8">
            <p className="text-lg text-gray-700 leading-relaxed text-center max-w-3xl mx-auto">
              Code.Connect is built to democratize software development by providing fast, reliable, and accessible 
              code compilation tools. We believe that every developer, regardless of their background or resources, 
              should have access to powerful development infrastructure that accelerates innovation and learning.
            </p>
          </div>
        </section>

        {/* Technology Stack */}
        <section className="mb-16">
          <div className="text-center mb-12">
            <h2 className="text-3xl font-bold text-black mb-4">Built With Modern Technology</h2>
            <div className="w-16 h-0.5 bg-black mx-auto"></div>
          </div>
          
          <div className="grid md:grid-cols-2 gap-8">
            <div className="bg-white border border-gray-200 rounded-lg p-6">
              <h3 className="text-xl font-semibold text-black mb-4">Frontend & Backend</h3>
              <ul className="space-y-3">
                <li className="flex items-center">
                  <div className="w-2 h-2 bg-black rounded-full mr-3"></div>
                  <span className="text-gray-700">Next.js for full-stack development</span>
                </li>
                <li className="flex items-center">
                  <div className="w-2 h-2 bg-black rounded-full mr-3"></div>
                  <span className="text-gray-700">Rust for high-performance compilation</span>
                </li>
                <li className="flex items-center">
                  <div className="w-2 h-2 bg-black rounded-full mr-3"></div>
                  <span className="text-gray-700">TypeScript for type safety</span>
                </li>
              </ul>
            </div>

            <div className="bg-white border border-gray-200 rounded-lg p-6">
              <h3 className="text-xl font-semibold text-black mb-4">Data & Infrastructure</h3>
              <ul className="space-y-3">
                <li className="flex items-center">
                  <div className="w-2 h-2 bg-black rounded-full mr-3"></div>
                  <span className="text-gray-700">PostgreSQL for robust data storage</span>
                </li>
                <li className="flex items-center">
                  <div className="w-2 h-2 bg-black rounded-full mr-3"></div>
                  <span className="text-gray-700">Prisma ORM for database management</span>
                </li>
                <li className="flex items-center">
                  <div className="w-2 h-2 bg-black rounded-full mr-3"></div>
                  <span className="text-gray-700">Docker & Docker Compose for deployment</span>
                </li>
              </ul>
            </div>
          </div>
        </section>

        {/* Features */}
        <section className="mb-16">
          <div className="text-center mb-12">
            <h2 className="text-3xl font-bold text-black mb-4">Why Code.Connect</h2>
            <div className="w-16 h-0.5 bg-black mx-auto"></div>
          </div>
          
          <div className="grid md:grid-cols-3 gap-6">
            <div className="text-center p-6 bg-gray-50 border border-gray-200 rounded-lg">
              <h3 className="text-lg font-semibold text-black mb-3">Lightning Fast</h3>
              <p className="text-gray-600">Rust-powered compilation engine delivers results in milliseconds, not minutes.</p>
            </div>
            <div className="text-center p-6 bg-gray-50 border border-gray-200 rounded-lg">
              <h3 className="text-lg font-semibold text-black mb-3">Secure & Reliable</h3>
              <p className="text-gray-600">Containerized execution environment ensures safe code compilation and execution.</p>
            </div>
            <div className="text-center p-6 bg-gray-50 border border-gray-200 rounded-lg">
              <h3 className="text-lg font-semibold text-black mb-3">Open Source</h3>
              <p className="text-gray-600">Built for the community, by the community. Transparent, extensible, and free.</p>
            </div>
          </div>
        </section>

        {/* Impact */}
        <section className="mb-16">
          <div className="bg-black text-white rounded-lg p-8">
            <div className="text-center">
              <h2 className="text-3xl font-bold mb-4">For the Benefit of Society</h2>
              <p className="text-lg leading-relaxed max-w-2xl mx-auto">
                We believe technology should serve humanity. Code.Connect removes barriers to software development, 
                enabling students, researchers, and innovators worldwide to build solutions that matter. 
                From educational institutions to open-source projects, we are committed to making 
                powerful development tools accessible to everyone.
              </p>
            </div>
          </div>
        </section>

        {/* Stats */}
        <section className="mb-16">
          <div className="grid grid-cols-3 gap-6 text-center">
            <div className="p-6">
              <div className="text-3xl font-bold text-black mb-2">2025</div>
              <div className="text-sm text-gray-600 uppercase tracking-wide">Project Started</div>
            </div>
            <div className="p-6">
              <div className="text-3xl font-bold text-black mb-2">10+</div>
              <div className="text-sm text-gray-600 uppercase tracking-wide">Languages Supported</div>
            </div>
            <div className="p-6">
              <div className="text-3xl font-bold text-black mb-2">Open</div>
              <div className="text-sm text-gray-600 uppercase tracking-wide">Source</div>
            </div>
          </div>
        </section>

        {/* Developer Section */}
        <section className="mb-16">
          <div className="text-center mb-8">
            <h2 className="text-3xl font-bold text-black mb-4">Developer</h2>
            <div className="w-16 h-0.5 bg-black mx-auto"></div>
          </div>
          
          <div className="max-w-2xl mx-auto">
            <div className="bg-gray-50 border border-gray-200 rounded-lg p-8 text-center">
              <div className="mb-6">
                <h3 className="text-xl font-semibold text-black mb-2">Built by a Passionate Developer</h3>
                <p className="text-gray-600">
                  This project represents countless hours of research, development, and testing. 
                  Every line of code is written with the goal of creating something meaningful 
                  for the developer community.
                </p>
              </div>
              
              <div className="space-y-2 text-sm">
                <p className="text-gray-600">
                  <span className="font-medium text-black">Focus:</span> Full-stack development, systems programming, DevOps
                </p>
                <p className="text-gray-600">
                  <span className="font-medium text-black">Mission:</span> Making development tools accessible to everyone
                </p>
                <p className="text-gray-600">
                  <span className="font-medium text-black">Vision:</span> A world where coding barriers dont limit innovation
                </p>
              </div>
            </div>
          </div>
        </section>

        {/* Call to Action */}
        <footer className="text-center pt-8 border-t border-gray-200">
          <div className="space-y-4">
            <h3 className="text-xl font-semibold text-black">Ready to start coding?</h3>
            <div className="flex justify-center gap-4">
              <Link 
                href="/compiler" 
                className="px-6 py-3 bg-black text-white rounded-lg hover:bg-gray-800 transition-colors font-medium"
              >
                Try the Compiler
              </Link>
              <Link 
                href="/" 
                className="px-6 py-3 border border-gray-300 text-black rounded-lg hover:bg-gray-50 transition-colors font-medium"
              >
                Back to Home
              </Link>
            </div>
          </div>
        </footer>
      </div>
    </div>
  );
}