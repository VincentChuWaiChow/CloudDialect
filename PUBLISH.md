# Publishing CloudDialect to GitHub

The intended remote is `https://github.com/VincentChuWaiChow/CloudDialect`.

## Important

- The files include **draft legal language** and unresolved `[PLACEHOLDERS]`.
- `LICENSE.md` is not yet an operative community grant. This starter includes no framework code.
- Review `PUBLISHING-CHECKLIST.md` before releasing framework source, accepting copyrighted contributions or signing commercial licenses.
- Do not accidentally call this an OSI-approved open-source project.

## Option A — GitHub website

1. Open https://github.com/new and create **CloudDialect** under **VincentChuWaiChow** with **Public** visibility.
2. Leave **Add a README**, **Add .gitignore** and **Choose a license** unset (these files are already in this starter).
3. On the empty repository, choose **uploading an existing file**.
4. Unzip the starter archive on your computer and upload its **contents**, preserving the directory structure.
5. Commit to `main` and verify the README, the image, and the draft notices.

## Option B — GitHub CLI (requires authenticated `gh` and `git`)

Run in the extracted `CloudDialect` directory:

```bash
git init -b main
git add .
git commit -m "chore: bootstrap CloudDialect branding and licensing drafts"
gh repo create VincentChuWaiChow/CloudDialect --public --source=. --remote=origin --push --description "CloudDialect framework: community access with commercial licensing"
```

Do not run this command if the remote repository already exists. In that case, add its remote and push only after verifying its existing history and contents.

## Final legal gate

The custom license is intended to require payment for covered use in revenue-generating products; it does **not** claim that a publisher automatically controls copyrights in third-party code, independent outputs or prior open-source releases. Have qualified IP counsel confirm its scope, enforceability, contract language and owner before activation.