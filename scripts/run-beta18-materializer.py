#!/usr/bin/env python3
from pathlib import Path
import runpy

path = Path('scripts/materialize-beta18.py')
text = path.read_text(encoding='utf-8')
old = '    "        ZeroKnowledgeValue::SimulatedResponse => format!(\\n",\n'
new = '    "        ZeroKnowledgeValue::Nonce | ZeroKnowledgeValue::SimulatedResponse => format!(\\n",\n'
assert text.count(old) == 1, text.count(old)
text = text.replace(old, new, 1)
old_tail = "        ZeroKnowledgeValue::SimulatedResponse => format!(\\n''',\n"
new_tail = "        ZeroKnowledgeValue::Nonce | ZeroKnowledgeValue::SimulatedResponse => format!(\\n''',\n"
assert text.count(old_tail) == 1, text.count(old_tail)
path.write_text(text.replace(old_tail, new_tail, 1), encoding='utf-8')
runpy.run_path(str(path), run_name='__main__')
