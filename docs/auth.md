# MINAS Auth Flows

## User / Pass

Standard predictable familiar to users. Doesn't need much more to this. Email
requires verification unless received an invitation via a direct email link.

* Stretch goal: overengineer and support adding 2FA :p

## Email auth link

For low privilege users that are only managing themselves we can avoid the full
acccount flow and use long-lived cookies and email auth links to login.

## User / Webauthn

Stretch goal do WebAuthn for authentication, falling back on email links.

## Password Resets

For simplicity for now they'll only be triggered by privileged users of the
system that are already logged in. Shouldn't be an issue as unprivileged users
will not necessarily have a password to lose.
