# COURSE.md — Supervised course mode

The README describes the default mode: a single learner, a personal fork, work on its own learn branch. In a supervised course setting with several students, forks are replaced by named student branches in this repository, and the instructor validates each learning step through a merge request. This file defines that mode — one way to run such a course; adopt whatever fits your own setting.

## Roles

- **Student:** implements the milestone placeholders of the SPECs, one milestone at a time, on working branches. Never touches main or the curriculum branch.
- **Instructor:** owns main and the curriculum branch, reviews every milestone PR, and is the only person who merges into a validated student branch. A merge is the validation seal.

## Branch model

| Branch | Purpose | Who merges |
|---|---|---|
| `main` | Scaffold plus documentation. The permanent reference. | Instructor only |
| `learn` | Curriculum branch: documentation changes only. | Instructor only |
| `learn/<name>` | One validated progress branch per student, seeded from main at course start. Never reset. | Instructor only |
| `<name>/m<x>` | Working branch for one milestone, deleted after its PR is merged. | Student |

`learn/<name>` is the student's ledger: every merged milestone lands there and nowhere else. The diff against main is the student's visible progress, at any moment, in the same way a fork's main works in solo mode.

## Workflow per milestone

1. Student creates the working branch from their validated branch:
   `git switch learn/<name>`, then `git switch -c <name>/m2`.
2. One SPEC section is the working prompt. Implement until every CI stage is green.
3. Student opens a pull request: base is `learn/<name>`, head is `<name>/m2`. The PR description states which acceptance criteria the milestone claims.
4. Student fills the journal entry for the milestone (misconception field, one-sentence self-explanation) before requesting review.
5. Instructor reviews against the checklist below, requests changes if needed, and merges. The merge is the validation; no other event is.
6. Student deletes the working branch and starts the next milestone from the updated `learn/<name>`.

## Validation checklist (instructor)

- Acceptance criteria of the milestone met, per the SPEC section.
- CI green, no skipped tests, no changes to SPECs or to shared files.
- Journal entry complete: misconception named in the student's own words, one-sentence explanation present.
- No implementation leakage into shared branches.

## Branch protection

- `main` and `learn`: no direct pushes; merges by the instructor only.
- `learn/<name>`: restrict direct pushes; merges through pull requests only. This makes the instructor's merge structurally necessary, a student cannot advance their validated state without validation.
- Per-student push restrictions: a student may push working branches, but the restriction on their validated branch stays with the instructor. At course start, the instructor creates each `learn/<name>` and pushes it once.

## Reading pointers and journals

Both travel with the course mode unchanged. The Reading column of the milestone tables names the literature for each milestone; journal files live inside each student's branch, so `learn/<name>` carries that student's journal history with it, and the instructor reads the journal as part of the review.

## The instructor as self-learner

An instructor who also walks the curriculum — ideally one milestone ahead of the course — keeps that work in a private copy of this repository: a clone pushed to the instructor's own GitHub or GitLab, fully decoupled from this repository, with this repository added as an upstream remote for pulling curriculum updates. It stays private because the instructor's implementations are worked solutions, and worked solutions inside a course repository would defeat the point of every student branch. Students never see the copy. How exactly it is set up — a GitHub duplicate that preserves history and tags, or a plain clone with a new remote — is the instructor's own decision.

## Seeding a student branch

```bash
git switch main
git switch -c learn/anna
git push -u origin learn/anna
```

Seeding happens once, at course start, from the current main. Late joiners are seeded from main as well; their diff target is same for everyone.
